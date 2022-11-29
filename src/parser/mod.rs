use roxmltree::{Document, Node};

use crate::types::{Coordinates, ErrorResponse, OjpDoc, OjpNode, Trip};

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
    Ok(ExchangePoint {
        place_ref: ojp_node
            .text_of("StopPlaceRef")
            .or_else(|_| ojp_node.text_of("StopPointRef"))?,
        location_name: ojp_node.text_tag_of("LocationName")?,
        coordinates: Coordinates {
            lng: ojp_node.text_of("Longitude")?.parse::<f64>()?,
            lat: ojp_node.text_of("Latitude")?.parse::<f64>()?,
        },
        // pt_mode: ojp_node.text_of("PtMode")?,
        pt_mode: "lol".to_string(),
    })
}

//The Result of a Trip Request gets parsed into a Trip Struct for further processing
// pub fn parse_trip(trip_result: &Node) -> Result<Trip, ErrorResponse> {
//     let ojp_node = OjpNode(trip_result);
//     Ok(Trip {
//         id: ojp_node.text_of("TripId")?,
//         duration: ojp_node.text_of("Duration")?,
//         start_time: ojp_node.text_of("StartTime")?,
//         end_time: ojp_node.text_of("EndTime")?,
//         transfers: ojp_node.text_of("Transfers")?,
//         legs: ojp_node
//             .0
//             .descendants()
//             .filter(|d| d.has_tag_name("TripLeg")),
//     })
// }

// fn parse_leg(leg: &Node) ->
