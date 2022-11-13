use super::*;

#[test]
fn test_test() {
    assert_eq!("hello", "hello");
}

#[test]
fn test_lir_parsing_ok() {
    assert_eq!(
        Ok(Location {
            stop_place_ref: "8507000".to_string(),
            stop_place_name: "Bern".to_string(),
            location_name: "Bern (Bern)".to_string(),
            coordinates: Coordinates {
                lat: 46.94883,
                lng: 7.43913
            }
        }),
        parse_lir(
            &roxmltree::Document::parse(
                r#"<Location>
    <Location>
      <StopPlace>
        <StopPlaceRef>8507000</StopPlaceRef>
        <StopPlaceName>
          <Text xml:lang="de">Bern</Text>
        </StopPlaceName>
        <TopographicPlaceRef>23006351:1</TopographicPlaceRef>
      </StopPlace>
      <LocationName>
        <Text xml:lang="de">Bern (Bern)</Text>
      </LocationName>
      <GeoPosition>
        <Longitude>7.43913</Longitude>
        <Latitude>46.94883</Latitude>
      </GeoPosition>
    </Location>
    <Complete>true</Complete>
    <Probability>1</Probability>
  </Location>"#
            )
            .unwrap()
            .root()
        )
    );
}
