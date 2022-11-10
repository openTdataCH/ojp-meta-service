use roxmltree::{Document, Node};

use crate::types::{Coordinates, ErrorResponse, OjpNode};

use super::types::Location;

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
