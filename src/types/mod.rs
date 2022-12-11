use std::{num::ParseFloatError, str::FromStr, vec};

use dotenvy_macro::dotenv;
use rocket::serde::json::Json;
use roxmltree::Node;
use serde::{Deserialize, Serialize};

// ------------- ERRORS --------------- //

#[derive(Debug, Responder, PartialEq)]
pub enum ErrorResponse {
    #[response(status = 500, content_type = "text")]
    ReqwestError(String),
    #[response(status = 500, content_type = "text")]
    ParseError(String),
    #[response(status = 400, content_type = "text")]
    SystemNotFoundError(String),
    #[response(status = 500, content_type = "json")]
    Generic(Json<ErrorMessage>),
}

impl ErrorResponse {
    pub fn generic(message: &str) -> Self {
        Self::Generic(Json(ErrorMessage {
            message: message.to_string(),
        }))
    }
}

#[derive(Debug, Serialize, Responder, PartialEq)]
pub struct ErrorMessage {
    pub message: String,
}

impl From<ParseFloatError> for ErrorResponse {
    fn from(source: ParseFloatError) -> Self {
        Self::ParseError(source.to_string())
    }
}

impl From<roxmltree::Error> for ErrorResponse {
    fn from(source: roxmltree::Error) -> Self {
        Self::ParseError(source.to_string())
    }
}

impl From<String> for ErrorResponse {
    fn from(source: String) -> Self {
        Self::ParseError(source.to_string())
    }
}

// ------------- SYSTEM --------------- //

//The Different Systems available.
#[derive(Debug, PartialEq, Serialize)]
pub enum System {
    CH,
    AT,
    IT,
    SLO,
}

//map easy identifiers to the different available systems. Error is received when the system doesn't exist.
impl FromStr for System {
    type Err = ErrorResponse;

    fn from_str(input: &str) -> Result<System, Self::Err> {
        match input {
            "ch" => Ok(System::CH),
            "at" => Ok(System::AT),
            "it" => Ok(System::IT),
            "slo" => Ok(System::SLO),
            x => Err(ErrorResponse::SystemNotFoundError(format!(
                "system with identifier {x} not found"
            ))),
        }
    }
}

//The different Systems get "built" with information from the .env File.
impl System {
    pub const fn get_config(&self) -> SystemConfig {
        match self {
            System::CH => SystemConfig {
                req_ref: dotenv!("CH_REQ_REF"),
                key: dotenv!("CH_KEY"),
                url: dotenv!("CH_URL"),
                id: System::CH,
            },
            System::AT => SystemConfig {
                req_ref: dotenv!("AT_REQ_REF"),
                key: dotenv!("AT_KEY"),
                url: dotenv!("AT_URL"),
                id: System::AT,
            },
            System::IT => SystemConfig {
                req_ref: dotenv!("IT_REQ_REF"),
                key: dotenv!("IT_KEY"),
                url: dotenv!("IT_URL"),
                id: System::IT,
            },
            System::SLO => SystemConfig {
                req_ref: dotenv!("SLO_REQ_REF"),
                key: dotenv!("SLO_KEY"),
                url: dotenv!("SLO_URL"),
                id: System::SLO,
            },
        }
    }

    pub const fn get_all() -> [System; 4] {
        [System::CH, System::AT, System::IT, System::SLO ]
    }

    // returns neighboring countries for a given system
    pub fn adjacent(&self) -> Vec<System> {
        match self {
            System::CH => vec![System::AT, System::IT],
            System::AT => vec![System::CH, System::IT, System::SLO],
            System::IT => vec![System::AT, System::CH, System::SLO],
            System::SLO => vec![System::AT, System::IT],
        }
    }
    // returns an array of matching neighboring countries for two systems
    pub fn shared_adjacency(&self, dest: System) -> Vec<System> {
        let mut shared_neighbors: Vec<System> = Vec::new();
        let origin: Vec<System> = self.adjacent();
        let dest: Vec<System> = dest.adjacent();
        for elem in origin {
            if dest.contains(&elem) {
                shared_neighbors.push(elem)
            }
        }
        return shared_neighbors;
    }
}

#[derive(Debug, PartialEq, Serialize)]
pub struct SystemConfig {
    pub req_ref: &'static str,
    pub key: &'static str,
    pub url: &'static str,
    pub id: System,
}
// ------------ State -------------//

//Struct where all the Exchange Points are being cached for faster access time.
pub struct ExchangePointState {
    pub ch: Vec<ExchangePoint>,
    pub at: Vec<ExchangePoint>,
    pub it: Vec<ExchangePoint>,
    pub slo: Vec<ExchangePoint>,
}

//The Cached Exchanged points all belong to their respective system and therefore have to be mapped
impl ExchangePointState {
    pub fn from_system(&self, sys: System) -> &Vec<ExchangePoint> {
        match sys {
            System::CH => &self.ch,
            System::AT => &self.at,
            System::IT => &self.it,
            System::SLO => &self.slo,
        }
    }
}

//Response. Id is needed to identify the system and xml is the result of an EPR.
pub struct ExchangePointResponse {
    pub id: System,
    pub xml: String,
}

// location information request
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct LocationInformationRequest {
    requestor_ref: Option<String>,
    location_name: Option<String>,
    // if someone wants to search "my location" this would be needed:
    coordinates: Option<Coordinates>,
    geo_restriction: Option<GeoRestriction>,
    nr_of_results: usize,
    // possibly, not sure yet
    // system: SystemRef,
}

// location information response
#[derive(Debug, PartialEq, Serialize)]
pub struct Location {
    pub stop_place_ref: String,
    pub stop_place_name: String,
    pub location_name: String,
    pub coordinates: Coordinates,
}

// trip request
pub struct TripRequest {
    origin: PlaceRef,
    destination: PlaceRef,
    // maybe we need to use chrono::DateTime for this, depends on what we need to do
    dep_arr_time: String,
    intermediate_stops: bool,
}

pub struct TripResult {
    id: String,
    trip: Trip,
}

pub struct Trip {
    id: String,
    duration: String,
    start_time: String,
    end_time: String,
    transfers: usize,
    legs: [TripLeg],
}

// not clear what this is, a trip seems to only ever have on leg what's the point of that
pub struct TripLeg {}

pub struct ExchangePointRequest {
    system: System,
    nr_of_results: usize,
    continue_at: usize,
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct ExchangePoint {
    pub place_ref: String,
    pub location_name: String,
    pub coordinates: Coordinates,
    pub pt_mode: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Coordinates {
    pub lat: f64,
    pub lng: f64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GeoRestriction {
    upper_left: Coordinates,
    lower_right: Coordinates,
}

struct PlaceRef {
    stop_place_ref: String,
    name: String,
}

// wrapper struct around roxmltree::Node so we can impl some methods
pub struct OjpNode<'a>(pub &'a Node<'a, 'a>);

impl OjpNode<'_> {
    pub fn tag_name(&self, name: &str) -> Result<Node, ErrorResponse> {
        Ok(self
            .0
            .descendants()
            .find(|n| n.has_tag_name(name))
            .ok_or(format!("<no node with tag name <{name}>"))?)
    }
    pub fn text_of(&self, name: &str) -> Result<String, ErrorResponse> {
        Ok(self
            .tag_name(name)?
            .text()
            .ok_or(format!("node with tag name <{name}> contains no text"))?
            .to_string())
    }

    pub fn text_tag_of(&self, name: &str) -> Result<String, ErrorResponse> {
        Ok(OjpNode(&OjpNode(&self.tag_name(name)?).0).text_of("Text")?)
    }
}
