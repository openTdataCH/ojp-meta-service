#![allow(dead_code)]
#![allow(unused_imports)]
#[cfg(test)]
mod tests;

mod parser;
mod requests;
mod types;

use std::str::FromStr;

use parser::*;
use requests::{format_epr, format_lir};
use reqwest::Client;
use rocket::fs::{relative, FileServer};
use rocket::futures::{stream, StreamExt};
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

// example route showing how to receive and unpack json
#[get("/exchange?<system>")]
fn exchange<'a>(
    system: &'a str,
    exchange_points: &'a State<ExchangePointState>,
) -> Result<Json<&'a Vec<ExchangePoint>>, ErrorResponse> {
    let sys = System::from_str(system)?;
    Ok(Json(exchange_points.from_system(sys)))
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
    // example reqwest
    // TODO use parallel requests to query multiple systems
    // TODO write a function for this stuff to keep routes clean
    let res = Client::new()
        .post(system.url)
        .bearer_auth(system.key)
        .header("Content-Type", "text/xml")
        .body(format_lir(query, 10, false))
        .send()
        .await
        .map_err(|_| ErrorResponse::ReqwestError("OJP-Service can't be reached...".to_string()))?
        .text()
        .await
        // with map_err we can map a reqwest error (which we can't control) to a custom error
        .map_err(|_| {
            ErrorResponse::ReqwestError("OJP-Service repsonse can't be read...".to_string())
        })?;
    let doc = roxmltree::Document::parse(&res).unwrap();
    let nodes = doc
        .descendants()
        .find(|n| n.has_tag_name("OJPLocationInformationDelivery"))
        .and_then(|f| {
            Some(
                f.children()
                    .filter(|n| n.has_tag_name("Location"))
                    .collect::<Vec<roxmltree::Node>>(),
            )
        })
        .unwrap();
    let locs = nodes
        .iter()
        .map(|n| parse_lir(&n))
        .collect::<Result<Vec<Location>, ErrorResponse>>()?;
    Ok(Json(locs))
}

//Get one location from every query
#[get("/location/<query>")]
async fn multiLocation(
    query: &str,
) -> Result<Json<Vec<MultiLIRResponse>, ErrorResponse>> {
    let nr_of_reqs: usize = 1;
    let client = Client::new();

    //gather all the configs for there different systems
    let system_configs: Vec<SystemConfig> =
        System::get_all().iter().map(|s| s.get_config()).collect();

    let bodies = stream::iter(system_configs)
    .map(|system| {
        let client = &client;
        async move {
            let resp = client
                .post(system.url)
                .bearer_auth(system.key)
                .header("Content-Type", "text/xml")
                .body(format_lir(query), 10, false)
                .send()
                .await
                .map_err(|_| ErrorResponse::ReqwestError("OJP-Service can't be reached...".to_string()))?
                .text()
                .await
                // with map_err we can map a reqwest error (which we can't control) to a custom error
                .map_err(|_| {
                    ErrorResponse::ReqwestError("OJP-Service response can't be read...".to_string())
                 })?;
                    
                let loc_xml = resp.text().await?;
            println!("{:?}", system.id);
            let result: Result<MultiLIRResponse, reqwest::Error> =
                Ok(MultiLIRResponse { id: system.id, loc_xml });
            result
        }
    })?;
    let doc = roxmltree::Document::parse(&bodies).unwrap();
    let nodes = doc
        .descendants()
        .find(|n| n.has_tag_name("OJPLocationInformationDelivery"))
        .and_then(|f| {
            Some(
                f.children()
                    .filter(|n| n.has_tag_name("Location"))
                    .collect::<Vec<roxmltree::Node>>(),
            )
        })
        .unwrap();
    let locs = nodes
        .iter()
        .map(|n| parse_lir(&n))
        .collect::<Result<Vec<Location>, ErrorResponse>>()?;
    Ok(Json(locs))
}


// index, perfect place to mount a demo application (via FileServer like openapi)
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}


//Entry Point of the application. Everything gets started here.
#[launch]
async fn rocket() -> _ {
    //number of requests for caching
    let nr_of_reqs: usize = 1;

    //the exchange points get cached in these vectors
    let exchange_points: ExchangePointState = ExchangePointState {
        ch: vec![],
        at: vec![],
        it: vec![],
        slo: vec![],
    };

    let client = Client::new();

    //gather all the configs for there different systems
    let system_configs: Vec<SystemConfig> =
        System::get_all().iter().map(|s| s.get_config()).collect();

    //get the exchange points from the different systems and return them as ExchangePointResponses
    let bodies = stream::iter(system_configs)
        .map(|system| {
            let client = &client;
            async move {
                let resp = client
                    .post(system.url)
                    .bearer_auth(system.key)
                    .header("Content-Type", "text/xml")
                    .body(format_epr(system.req_ref))
                    .send()
                    .await?;
                let xml = resp.text().await?;
                println!("{:?}", system.id);
                let result: Result<ExchangePointResponse, reqwest::Error> =
                    Ok(ExchangePointResponse { id: system.id, xml });
                result
            }
        })
        .buffer_unordered(nr_of_reqs);

    //Map the result to structs
    bodies
        .for_each(|res| async {
            match res {
                // parse exchange points and write them into exchpnt struct
                Ok(_res) => println!("unpack res here"),
                Err(e) => eprintln!("Got an error writing exchange points: {}", e),
            }
        })
        .await;

    //build the app
    rocket::build()
        .mount("/", routes![index, location, system, echo, exchange])
        .mount("/docs", FileServer::from(relative!("/docs")))
        .manage(exchange_points)
}
