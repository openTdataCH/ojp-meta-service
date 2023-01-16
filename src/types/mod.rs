use std::{
    collections::HashMap,
    num::{ParseFloatError, ParseIntError},
    str::FromStr,
    vec,
};

use chrono::{DateTime, NaiveDateTime};
use dotenvy_macro::dotenv;
use rocket::serde::json::Json;
use roxmltree::{Document, Node};
use serde::{Deserialize, Serialize};

use crate::parser::{parse_epr, parse_lir, parse_trip};

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

impl From<ParseIntError> for ErrorResponse {
    fn from(source: ParseIntError) -> Self {
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
#[derive(Debug, PartialEq, Serialize, Clone, Copy, Hash, PartialOrd, Eq, Ord)]
pub enum System {
    CH,
    AT,
    IT,
    SLO,
    FERN,
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
            "fern" => Ok(System::FERN),
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
            System::FERN => SystemConfig {
                req_ref: dotenv!("FERN_REQ_REF"),
                key: dotenv!("FERN_KEY"),
                url: dotenv!("FERN_URL"),
                id: System::FERN,
            },
        }
    }

    pub const fn get_exp_systems() -> [System; 4] {
        [System::CH, System::AT, System::IT, System::SLO]
    }

    // returns neighboring countries for a given system
    pub fn adjacent(&self) -> Vec<System> {
        match self {
            System::CH => vec![System::AT, System::IT],
            System::AT => vec![System::CH, System::IT, System::SLO],
            System::IT => vec![System::AT, System::CH, System::SLO],
            System::SLO => vec![System::AT, System::IT],
            System::FERN => System::get_exp_systems().to_vec(),
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

    pub fn ajdacent_paths(&self, dest: System) -> Vec<Adjacency> {
        let indirect = &mut self
            .adjacent()
            .into_iter()
            .filter(|s| dest.adjacent().contains(s))
            .map(|s| Adjacency::Indirect(*self, s, dest))
            .collect::<Vec<Adjacency>>();

        match &self.adjacent().contains(&dest) {
            true => {
                indirect.push(Adjacency::Direct(*self, dest));
                return indirect.to_vec();
            }
            false => indirect.to_vec(),
        }
    }
}

//Values belonging to the systems. Requestor reference, a key for authentication, an url and an ID.
#[derive(Debug, PartialEq, Serialize)]
pub struct SystemConfig {
    pub req_ref: &'static str,
    pub key: &'static str,
    pub url: &'static str,
    pub id: System,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Adjacency {
    Direct(System, System),
    Indirect(System, System, System),
}
// ------------ State -------------//

//Struct where all the Exchange Points are being cached for faster access time.
#[derive(Debug)]
pub struct ExchangePointState(pub HashMap<System, Vec<ExchangePoint>>);

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
    origin: String,
    destination: String,
    // maybe we need to use chrono::DateTime for this, depends on what we need to do
    dep_arr_time: String,
    intermediate_stops: bool,
}

#[derive(Debug, Serialize)]
pub struct Trip {
    pub id: String,
    pub duration: String,
    pub start_time: String,
    pub end_time: String,
    pub transfers: String,
    pub legs: Vec<TripLeg>,
}

#[derive(Debug, Serialize)]
pub enum TripLeg {
    // this might need a more complex type that captures metadata, but it's ok for now
    TimedLeg(Vec<TimedLeg>),
    TransferLeg(TransferLeg),
}

#[derive(Debug, Serialize)]
pub enum TimedLegType {
    Board,
    Intermediate,
    Alight,
}

impl FromStr for TimedLegType {
    type Err = ErrorResponse;

    fn from_str(input: &str) -> Result<TimedLegType, Self::Err> {
        match input {
            "LegBoard" => Ok(TimedLegType::Board),
            "LegIntermediates" => Ok(TimedLegType::Intermediate),
            "LegAlight" => Ok(TimedLegType::Alight),

            x => Err(ErrorResponse::ParseError(format!(
                "leg type with identifier {x} not found"
            ))),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct TimedLeg {
    pub stop_point_ref: String,
    pub stop_point_name: String,
    pub planned_quay: Option<String>,
    pub departure_time: String,
    // pub order: u32,
    pub kind: TimedLegType,
}

#[derive(Debug, Serialize)]
pub struct TransferLeg {
    pub mode: String,
    pub start_point_ref: String,
    pub start_location_name: String,
    pub stop_point_ref: String,
    pub stop_location_name: String,
    pub start_time: String,
    pub duration: String,
    pub walk_duration: Option<String>,
}

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
    pub pt_mode: Option<String>,
    pub private_code: String,
}

pub struct Point {
    pub place_ref: String,
    pub place_name: String,
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

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TripForm {
    pub origin: TripLocation,
    pub destination: TripLocation,
    pub exchange: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TripLocation {
    pub name: String,
    pub reference: String,
    pub system: String,
}

// wrapper struct around roxmltree::Node so we can impl some methods
#[derive(Debug)]
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

    pub fn contains(&self, name: &str) -> bool {
        match &self.0.descendants().find(|n| n.has_tag_name(name)) {
            Some(_) => true,
            None => false,
        }
    }

    pub fn contains_which<'a>(&'a self, names: Vec<&'a str>) -> Option<(&Self, &str)> {
        match names.iter().find(|name| self.contains(name)) {
            Some(f) => Some((self, *f)),
            None => None,
        }
    }

    pub fn contains_either_val<'a>(
        &'a self,
        descendant: &'a str,
        containee: &'a str,
        values: [&'a str; 2],
    ) -> Result<Node, ErrorResponse> {
        let containers = self
            .0
            .descendants()
            .filter(|n| n.has_tag_name(descendant))
            .collect::<Vec<Node>>();
        let correct = containers.iter().find(|c| {
            c.descendants()
                .find(|cc| {
                    OjpNode(cc)
                        .text_of(containee)
                        .ok()
                        .eq(&Some(values[0].to_string()))
                        || OjpNode(cc)
                            .text_of(containee)
                            .ok()
                            .eq(&Some(values[1].to_string()))
                })
                .is_some()
        });
        match correct {
            Some(x) => Ok(*x),
            None => Err(ErrorResponse::ParseError("Fuck this".to_string())),
        }
    }
}

pub struct OjpDoc<'a>(pub Document<'a>);

impl<'a> OjpDoc<'a> {
    pub fn new(xml: &'a str) -> Result<Self, ErrorResponse> {
        let ojp = OjpDoc(Document::parse(xml)?);
        Ok(ojp)
    }

    pub fn get_locations(&self) -> Result<Vec<Location>, ErrorResponse> {
        let locations = self
            .0
            .descendants()
            .find(|n| n.has_tag_name("OJPLocationInformationDelivery"))
            .ok_or("No location delivery node found".to_string())?
            .children()
            .filter(|n| n.has_tag_name("Location"))
            .collect::<Vec<Node>>();
        match locations.len() {
            0 => Err("No Locations found".to_string())?,
            _ => Ok(locations
                .iter()
                .map(|l| parse_lir(l))
                .collect::<Result<Vec<Location>, ErrorResponse>>()?),
        }
    }

    pub fn get_exchange_points(&self) -> Result<Vec<ExchangePoint>, ErrorResponse> {
        let expts = self
            .0
            .descendants()
            .find(|n| n.has_tag_name("OJPExchangePointsDelivery"))
            .ok_or("No exchange point delivery node found".to_string())?
            .children()
            .filter(|n| n.has_tag_name("Place"))
            .collect::<Vec<Node>>();
        match expts.len() {
            0 => Err("No Exchange Points found".to_string())?,
            _ => Ok(expts
                .iter()
                .map(|e| parse_epr(e))
                .collect::<Result<Vec<ExchangePoint>, ErrorResponse>>()?),
        }
    }

    pub fn get_trips(&self) -> Result<Vec<Trip>, ErrorResponse> {
        let trips = self
            .0
            .descendants()
            .filter(|n| n.has_tag_name("TripResult"))
            .collect::<Vec<Node>>();
        match trips.len() {
            0 => Err("No Trips found".to_string())?,
            _ => Ok(trips
                .iter()
                .map(|t| parse_trip(t))
                .collect::<Result<Vec<Trip>, ErrorResponse>>()?),
        }
    }
}
