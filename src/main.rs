#![allow(dead_code)]
#![allow(unused_imports)]
#[cfg(test)]
mod tests;

mod parser;
mod requests;
mod types;

use std::collections::HashMap;
use std::str::FromStr;

use parser::*;
use requests::{format_epr, format_lir, format_trip};
use reqwest::Client;
use rocket::fs::{relative, FileServer};
use rocket::futures::{stream, StreamExt, TryStreamExt};
use rocket::http::ext::IntoCollection;
use rocket::serde::json::Json;
use rocket::State;
use types::*;

use crate::requests::get_trip;

#[macro_use]
extern crate rocket;

// get exchange points of a system
#[get("/exchange?<system>")]
fn exchange<'a>(
    system: &'a str,
    exchange_points: &'a State<ExchangePointState>,
) -> Result<Json<&'a Vec<ExchangePoint>>, ErrorResponse> {
    let sys = System::from_str(system)?;
    Ok(Json(exchange_points.0.get(&sys).unwrap()))
}

// handler to query a location request
#[get("/location/<query>?<system>&<_lat>&<_lng>")]
async fn location(
    query: &str,
    system: &str,
    _lat: Option<&str>,
    _lng: Option<&str>,
) -> Result<Json<Vec<Location>>, ErrorResponse> {
    let system = System::from_str(system)?.get_config();
    let res = Client::new()
        .post(system.url)
        .bearer_auth(system.key)
        .header("Content-Type", "text/xml")
        .body(format_lir(query, system.req_ref, 10, false))
        .send()
        .await
        .or(Err("OJP-Service can't be reached...".to_string()))?
        .text()
        .await
        .or(Err("OJP response not readable...".to_string()))?;
    let doc = OjpDoc::new(&res)?;
    let locations = doc.get_locations()?;
    Ok(Json(locations))
}

// hardcoded trip request between two systems
#[get("/exchange_test")]
async fn exchange_test<'a>(
    exchange_points: &'a State<ExchangePointState>,
) -> Result<Json<(Vec<Trip>, Vec<Trip>)>, ErrorResponse> {
    let (origin, destination) = (
        System::from_str("ch")?.get_config(),
        System::from_str("at")?.get_config(),
    );

    let exchange_point_id = "ch:1:sloid:3000".to_string();

    let (origin_exp, destination_exp) = (
        exchange_points
            .0
            .get(&origin.id)
            .ok_or("No exchange points for this system".to_string())?
            .into_iter()
            .find(|n| n.private_code.eq(&exchange_point_id)),
        exchange_points
            .0
            .get(&destination.id)
            .ok_or("No exchange points for this system".to_string())?
            .into_iter()
            .find(|n| n.private_code.eq(&exchange_point_id)),
    );

    let first_trip = match origin_exp {
        Some(exp) => {
            get_trip(
                "8507000",
                "Bern",
                &exp.place_ref,
                &exp.location_name,
                origin,
            )
            .await
        }
        None => Err(ErrorResponse::ReqwestError("No trip".to_string())),
    }?;

    let second_trip = match destination_exp {
        Some(exp) => {
            get_trip(
                &exp.place_ref,
                &exp.location_name,
                "U3xBPTFATz1XaWVuIEhhdXB0YmFobmhvZkBYPTE2Mzc2NDEzQFk9NDgxODUxODRAVT04MUBMPTQ5MDEzNDkwMEBCPTFAcD0xNjY1MTMyOTEyQGk9QcOXYXQ6NDk6MTM0OUB8V2llbiBIYXVwdGJhaG5ob2Z8MTYuMzc2NDEzfDQ4LjE4NTE4NHx0cnVl",
                "Wien Hauptbahnhof",
                destination,
            )
            .await
        }
        None => Err(ErrorResponse::ReqwestError("No trip".to_string())),
    }?;

    Ok(Json((first_trip, second_trip)))
}

// trip request between two adjacent systems
#[post("/exchange_test_post", format = "json", data = "<request>")]
async fn exchange_test_post<'a>(
    request: Json<TripForm>,
    exchange_points: &'a State<ExchangePointState>,
) -> Result<Json<(Vec<Trip>, Vec<Trip>)>, ErrorResponse> {
    let data = request.into_inner();

    let (origin, destination) = (
        System::from_str(&data.origin.system)?.get_config(),
        System::from_str(&data.destination.system)?.get_config(),
    );

    let (origin_exp, destination_exp) = (
        exchange_points
            .0
            .get(&origin.id)
            .ok_or("No exchange points for this system".to_string())?
            .into_iter()
            .find(|n| n.private_code.eq(&data.exchange)),
        exchange_points
            .0
            .get(&destination.id)
            .ok_or("No exchange points for this system".to_string())?
            .into_iter()
            .find(|n| n.private_code.eq(&data.exchange)),
    );

    let first_trip = match origin_exp {
        Some(exp) => {
            get_trip(
                &data.origin.reference,
                &data.origin.name,
                &exp.place_ref,
                &exp.location_name,
                origin,
            )
            .await
        }
        None => Err(ErrorResponse::ReqwestError("No trip".to_string())),
    }?;

    let second_trip = match destination_exp {
        Some(exp) => {
            get_trip(
                &exp.place_ref,
                &exp.location_name,
                &data.destination.reference,
                &data.destination.name,
                destination,
            )
            .await
        }
        None => Err(ErrorResponse::ReqwestError("No trip".to_string())),
    }?;

    Ok(Json((first_trip, second_trip)))
}

// example trip request endpoint
#[get("/trip")]
async fn trip() -> Result<Json<Vec<Trip>>, ErrorResponse> {
    let system = System::from_str("ch")?.get_config();
    let res = Client::new()
        .post(system.url)
        .bearer_auth(system.key)
        .header("Content-Type", "text/xml")
        .body(format_trip(
            "8507000",
            "Bern",
            "8507380",
            "Grindelwald (Grindelwald)",
            "API-Explorer",
        ))
        .send()
        .await
        .or(Err("OJP-Service can't be reached...".to_string()))?
        .text()
        .await
        .or(Err("OJP response not readable...".to_string()))?;
    let doc = OjpDoc::new(&res)?;
    let trips = doc.get_trips()?;
    Ok(Json(trips))
}
// index, perfect place to mount a demo application (via FileServer like openapi)
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

//Entry Point of the application. Everything gets started here.
#[launch]
async fn rocket() -> _ {
    let client = Client::new();

    //gather all the configs for there different systems
    let system_configs: Vec<SystemConfig> = System::get_exp_systems()
        .iter()
        .map(|s| s.get_config())
        .collect();

    // send all the exchange points request asynchronously
    let exchange_points = stream::iter(system_configs)
        .map(|system| {
            let client = &client;
            async move {
                let resp = client
                    .post(system.url)
                    .bearer_auth(system.key)
                    .header("Content-Type", "text/xml")
                    .body(format_epr(system.req_ref))
                    .send()
                    .await
                    .unwrap();
                let xml = resp.text().await.unwrap();
                let result = ExchangePointResponse { id: system.id, xml };
                result
            }
        })
        .buffer_unordered(4)
        .collect::<Vec<ExchangePointResponse>>()
        .await;

    // parse answers into exchange point state
    let exp_pts = ExchangePointState(
        exchange_points
            .iter()
            .map(|e| {
                (
                    e.id,
                    OjpDoc::new(&e.xml).unwrap().get_exchange_points().unwrap(),
                )
            })
            .collect(),
    );

    //mount the app
    rocket::build()
        .mount(
            "/",
            routes![
                index,
                location,
                exchange,
                trip,
                exchange_test,
                exchange_test_post
            ],
        )
        .mount("/docs", FileServer::from(relative!("/docs")))
        .manage(exp_pts)
}
