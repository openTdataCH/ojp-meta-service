use std::str::FromStr;

use rocket::{form::Error, http::ext::IntoCollection};
use roxmltree::{Descendants, Document, Node};

use crate::types::{
    Coordinates, ErrorResponse, OjpDoc, OjpNode, TimedLeg, TimedLegType, TransferLeg, Trip, TripLeg,
};

use super::types::{ExchangePoint, Location};

//The Result of a Location Information Request gets parsed into a Location Object for further processing
pub fn parse_lir(location: &Node) -> Result<Location, ErrorResponse> {
    let ojp_node = OjpNode(location);
    Ok(Location {
        stop_place_ref: ojp_node.text_of("StopPlaceRef")?,
        stop_place_name: ojp_node.text_tag_of("StopPlaceName")?,
        location_name: ojp_node.text_tag_of("LocationName")?,
        coordinates: Coordinates {
            lng: ojp_node.text_of("Longitude")?.parse::<f64>()?,
            lat: ojp_node.text_of("Latitude")?.parse::<f64>()?,
        },
    })
}

//The Result of an Exchange Point Request gets parsed into an ExchangePoint Object for further processing
pub fn parse_epr(exchange_point: &Node) -> Result<ExchangePoint, ErrorResponse> {
    let ojp_node = OjpNode(exchange_point);
    ojp_node.contains_either_val(
        "PrivateCode",
        "System",
        ["LA-ExchangePoint-ID", "LinkingAlps"],
    );
    Ok(ExchangePoint {
        place_ref: ojp_node
            .text_of("StopPlaceRef")
            .or_else(|_| ojp_node.text_of("StopPointRef"))?,
        location_name: ojp_node.text_tag_of("LocationName")?,
        coordinates: Coordinates {
            lng: ojp_node.text_of("Longitude")?.parse::<f64>()?,
            lat: ojp_node.text_of("Latitude")?.parse::<f64>()?,
        },
        pt_mode: ojp_node.text_of("PtMode").ok(),
        private_code: OjpNode(&ojp_node.contains_either_val(
            "PrivateCode",
            "System",
            ["LA-ExchangePoint-ID", "LinkingAlps"],
        )?)
        .text_of("Value")?,
    })
}

// The Result of a Trip Request gets parsed into a Trip Struct for further processing
pub fn parse_trip(trip_result: &Node) -> Result<Trip, ErrorResponse> {
    let ojp_node = OjpNode(trip_result);
    Ok(Trip {
        id: ojp_node.text_of("TripId")?,
        duration: ojp_node.text_of("Duration")?,
        start_time: ojp_node.text_of("StartTime")?,
        end_time: ojp_node.text_of("EndTime")?,
        transfers: ojp_node.text_of("Transfers")?,
        legs: parse_trip_legs(&trip_result)?,
    })
}

pub fn parse_trip_legs(node: &Node) -> Result<Vec<TripLeg>, ErrorResponse> {
    let legs = node
        .descendants()
        .filter(|n| n.has_tag_name("TripLeg"))
        .map(
            |n| match OjpNode(&n).contains_which(vec!["TimedLeg", "TransferLeg"]) {
                Some(leg) => match leg.1 {
                    "TimedLeg" => parse_timed_leg(leg.0),
                    "TransferLeg" => parse_transfer_leg(leg.0),
                    _ => Err(ErrorResponse::ParseError("no leg".to_string())),
                },
                None => Err(ErrorResponse::ParseError("no leg".to_string())),
            },
        )
        .collect();
    legs
}

fn parse_timed_leg(node: &OjpNode) -> Result<TripLeg, ErrorResponse> {
    Ok(TripLeg::TimedLeg(
        node.0
            .descendants()
            .find(|n| n.has_tag_name("TimedLeg"))
            .ok_or("can't parse timedleg".to_string())?
            .children()
            .filter(|n| match_any(n, &["LegBoard", "LegIntermediates", "LegAlight"]))
            .map(|c| {
                let timed_leg = OjpNode(&c);
                Ok(TimedLeg {
                    stop_point_ref: timed_leg.text_of("StopPointRef")?,
                    stop_point_name: timed_leg.text_tag_of("StopPointName")?,
                    planned_quay: timed_leg.text_tag_of("PlannedQuay").ok(),
                    departure_time: timed_leg.text_of("TimetabledTime")?,
                    // order: timed_leg.text_of("Order")?.parse::<u32>()?,
                    kind: TimedLegType::from_str(c.tag_name().name())?,
                })
            })
            .collect::<Result<Vec<TimedLeg>, ErrorResponse>>()?,
    ))
}

fn parse_transfer_leg(leg: &OjpNode) -> Result<TripLeg, ErrorResponse> {
    let start_node = leg
        .0
        .descendants()
        .find(|n| n.has_tag_name("LegStart"))
        .ok_or("LegStart not found".to_string())?;
    let start = OjpNode(&start_node);

    let end_node = leg
        .0
        .descendants()
        .find(|n| n.has_tag_name("LegEnd"))
        .ok_or("LegEnd not found".to_string())?;
    let end = OjpNode(&end_node);

    Ok(TripLeg::TransferLeg(TransferLeg {
        mode: leg.text_of("TransferMode")?,
        start_point_ref: start.text_of("StopPointRef")?,
        start_location_name: start.text_tag_of("LocationName")?,
        stop_point_ref: end.text_of("StopPointRef")?,
        stop_location_name: end.text_tag_of("LocationName")?,
        start_time: leg.text_of("TimeWindowStart")?,
        duration: leg.text_of("Duration")?,
        walk_duration: leg.text_of("WalkDuration").ok(),
    }))
}

fn match_any(node: &Node, names: &[&str]) -> bool {
    names
        .iter()
        .filter(|name| node.has_tag_name(**name))
        .collect::<Vec<&&str>>()
        .len()
        > 0
}
