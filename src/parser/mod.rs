use roxmltree::{Document, Node};

use crate::types::{Coordinates, ErrorResponse, OjpNode};

use super::types::{Location, ExchangePoint};

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

pub fn parse_epr(exchange_point: &Node) -> Result<ExchangePoint, ErrorResponse> {
    let ojp_node = OjpNode(exchange_point);
    Ok(ExchangePoint {
        place_ref: ojp_node.text_of("StopPlaceRef")?,
        location_name: ojp_node.text_tag_of("LocationName")?,
        coordinates: Coordinates {
            lng: ojp_node.text_of("Longitude")?.parse::<f64>()?,
            lat: ojp_node.text_of("Latitude")?.parse::<f64>()?,
        },
        pt_mode: ojp_node.text_of("PtMode")?,
    })
}
