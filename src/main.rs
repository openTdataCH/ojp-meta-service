#![allow(dead_code)]
#![allow(unused_imports)]
#[cfg(test)]
mod tests;

mod parser;
mod requests;
mod types;

use std::str::FromStr;

use parser::*;
use requests::format_lir;
use reqwest::Client;
use rocket::fs::{relative, FileServer};
use rocket::serde::json::Json;
use types::*;

#[macro_use]
extern crate rocket;

// example route showing how to receive and unpack json
#[post("/echo", data = "<lir>")]
fn echo(lir: Json<LocationInformationRequest>) -> Json<LocationInformationRequest> {
    Json(lir.into_inner())
}

// example route showing how to back propagate error and request system
#[get("/system/<id>")]
fn system(id: &str) -> Result<Json<SystemConfig>, ErrorResponse> {
    Ok(Json(System::from_str(id)?.get_config()))
}

// handler to query a location request
#[get("/location/<query>?<system>&<lat>&<lng>")]
async fn location(
    query: &str,
    system: &str,
    lat: Option<&str>,
    lng: Option<&str>,
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
        .map(|n| parse_lir_v2(&n))
        .collect::<Result<Vec<Location>, ErrorResponse>>()?;
    Ok(Json(locs))
}

// index, perfect place to mount a demo application (via FileServer like openapi)
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, location, system, echo])
        .mount("/docs", FileServer::from(relative!("/docs")))
}
