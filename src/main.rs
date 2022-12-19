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

#[macro_use]
extern crate rocket;

// example route showing how to receive and unpack json
#[post("/echo", data = "<lir>")]
fn echo(lir: Json<LocationInformationRequest>) -> Json<LocationInformationRequest> {
    Json(lir.into_inner())
}

// get exchange points of a system
#[get("/exchange?<system>")]
fn exchange<'a>(
    system: &'a str,
    exchange_points: &'a State<ExchangePointState>,
) -> Result<Json<&'a Vec<ExchangePoint>>, ErrorResponse> {
    let sys = System::from_str(system)?;
    Ok(Json(exchange_points.0.get(&sys).unwrap()))
}

// example route showing how to back propagate error and request system
#[get("/system/<id>")]
fn system(id: &str) -> Result<Json<SystemConfig>, ErrorResponse> {
    Ok(Json(System::from_str(id)?.get_config()))
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
    //call system get_exp_systems() to get all the systems and filter system::Fern

    let system_configs: Vec<SystemConfig> = System::get_exp_systems()
        .iter()
        .map(|s| s.get_config())
        .collect();

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

    //build the app
    rocket::build()
        .mount("/", routes![index, location, system, echo, exchange, trip])
        .mount("/docs", FileServer::from(relative!("/docs")))
        .manage(exp_pts)
}
