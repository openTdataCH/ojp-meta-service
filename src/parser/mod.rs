use roxmltree::{Document, Node};

use crate::types::{Coordinates, ErrorResponse};

use super::types::Location;

pub fn parse_lir(location: &Node) -> Result<Location, ErrorResponse> {
    // NOTE: this is dangerous, to reuse iterator everything has to be parsed in the right order
    // we might adapt this in the future
    let mut desc = location.descendants();
    Ok(Location {
        stop_place_ref: desc
            .find(|n| n.has_tag_name("StopPlaceRef"))
            // all this error handling should eventually move out of here
            // we'll do it like this for now
            .ok_or(ErrorResponse::ParseError(
                "no item with tag <StopPlaceRef>".to_string(),
            ))?
            .text()
            .ok_or(ErrorResponse::ParseError(
                "text of <StopPlaceRef> can't be parsed".to_string(),
            ))?
            .to_string(),
        stop_place_name: desc
            .find(|n| n.has_tag_name("StopPlaceName"))
            .ok_or(ErrorResponse::ParseError(
                "no item with tag <StopPlaceName>".to_string(),
            ))?
            .descendants()
            .find(|n| n.has_tag_name("Text"))
            .ok_or(ErrorResponse::ParseError(
                "no item with tag <Text> inside <StopPlaceName>".to_string(),
            ))?
            .text()
            .ok_or(ErrorResponse::ParseError(
                "text of <Text> inside <StopPlaceRef> can't be parsed".to_string(),
            ))?
            .to_owned(),
        location_name: desc
            .find(|n| n.has_tag_name("LocationName"))
            .ok_or(ErrorResponse::ParseError(
                "no item with tag <LocationName>".to_string(),
            ))?
            .descendants()
            .find(|n| n.has_tag_name("Text"))
            .ok_or(ErrorResponse::ParseError(
                "no item with tag <Text> inside <LocationName>".to_string(),
            ))?
            .text()
            .ok_or(ErrorResponse::ParseError(
                "text of <LocationName> can't be parsed".to_string(),
            ))?
            .to_string(),
        coordinates: Coordinates {
            lng: desc
                .find(|n| n.has_tag_name("Longitude"))
                .ok_or(ErrorResponse::ParseError(
                    "no item with tag <Longitude>".to_string(),
                ))?
                .text()
                .ok_or(ErrorResponse::ParseError(
                    "text of <LocationName> can't be parsed".to_string(),
                ))?
                .parse::<f64>()
                .map_err(|_| {
                    ErrorResponse::ParseError("can't parse <Longitude> to float".to_string())
                })?,
            lat: desc
                .find(|n| n.has_tag_name("Latitude"))
                .ok_or(ErrorResponse::ParseError(
                    "no item with tag <Latitude>".to_string(),
                ))?
                .text()
                .ok_or(ErrorResponse::ParseError(
                    "text of <LocationName> can't be parsed".to_string(),
                ))?
                .parse::<f64>()
                .map_err(|_| {
                    ErrorResponse::ParseError("can't parse <Latitude> to float".to_string())
                })?,
        },
    })
}
