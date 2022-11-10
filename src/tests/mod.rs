use super::*;

#[test]
fn test_lir_parsing_nok() {
    assert_eq!(
        Err("whoops".to_string()),
        parse_lir("something something ojp")
    );
}

#[test]
fn test_lir_parsing_ok() {
    assert_eq!(
        Ok(Location {
            stop_place_ref: "8507000",
            stop_place_name: "Bern",
            location_name: "Bern (Bern)",
            coordinates: Coordinates {
                lat: 46.94883,
                lng: 7.43913
            }
        }),
        parse_lir(
            r#"<ojp:Location>
    <ojp:Location>
      <ojp:StopPlace>
        <ojp:StopPlaceRef>8507000</ojp:StopPlaceRef>
        <ojp:StopPlaceName>
          <ojp:Text xml:lang="de">Bern</ojp:Text>
        </ojp:StopPlaceName>
        <ojp:TopographicPlaceRef>23006351:1</ojp:TopographicPlaceRef>
      </ojp:StopPlace>
      <ojp:LocationName>
        <ojp:Text xml:lang="de">Bern (Bern)</ojp:Text>
      </ojp:LocationName>
      <ojp:GeoPosition>
        <siri:Longitude>7.43913</siri:Longitude>
        <siri:Latitude>46.94883</siri:Latitude>
      </ojp:GeoPosition>
    </ojp:Location>
    <ojp:Complete>true</ojp:Complete>
    <ojp:Probability>1</ojp:Probability>
  </ojp:Location>"#
        )
    );
}
