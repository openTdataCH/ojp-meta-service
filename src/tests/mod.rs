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

#[test]
fn test_epr_parsing_ok() {
    assert_eq!(
        Ok(ExchangePoint {
            place_ref: "1300033".to_string(),
            location_name: "Aosta".to_string(),
            coordinates: Coordinates {
                lat: 45.73516,
                lng: 7.32432
            },
            pt_mode: "bus".to_string(),
        }),
        parse_epr(
            &roxmltree::Document::parse(
                r#"<Place>
    <Place>
      <StopPlace>
        <StopPlaceRef>1300033</StopPlaceRef>
        <StopPlaceName>
          <Text xml:lang="de">Aosta, Autostazione</Text>
        </StopPlaceName>
        <PrivateCode>
          <System>EFA</System>
          <Value>173775</Value>
        </PrivateCode>
        <PrivateCode>
          <System>LA-ExchangePoint-ID</System>
          <Value>it:itc:StopPlace:ITRP-ST0070039001:</Value>
        </PrivateCode>
        <TopographicPlaceRef>22007003:1</TopographicPlaceRef>
      </StopPlace>
      <LocationName>
        <Text xml:lang="de">Aosta</Text>
      </LocationName>
      <GeoPosition>
        <Longitude>7.32432</Longitude>
        <Latitude>45.73516</Latitude>
      </GeoPosition>
      <Extension>
        <PlaceExtensionStructure>
          <ExchangePoint>
            <WaitingTime>PT5M</WaitingTime>
          </ExchangePoint>
        </PlaceExtensionStructure>
      </Extension>
    </Place>
    <BorderPoint>false</BorderPoint>
    <Mode>
      <PtMode>bus</PtMode>
      <BusSubmode>localBusService</BusSubmode>
    </Mode>
  </Place>"#
            )
            .unwrap()
            .root()
        )
    );
}

#[test]
fn test_adjacent() {
    assert_eq!(
        System::AT.adjacent(),
        vec![System::CH, System::IT, System::SLO]
    );
    assert_eq!(System::SLO.adjacent(), vec![System::AT, System::IT]);
}

#[test]
fn test_shred_adjacency() {
    assert_eq!(
        System::AT.shared_adjacency(System::IT),
        vec![System::CH, System::SLO]
    );
    assert_eq!(
        System::CH.shared_adjacency(System::SLO),
        vec![System::AT, System::IT]
    );
}

#[test]
fn test_adjacent_paths() {
    assert_eq!(
        System::AT.ajdacent_paths(System::IT).sort(),
        vec![
            Adjacency::Indirect(System::AT, System::CH, System::IT),
            Adjacency::Indirect(System::AT, System::SLO, System::IT),
            Adjacency::Direct(System::AT, System::IT),
        ]
        .sort()
    );
    assert_eq!(
        System::CH.ajdacent_paths(System::SLO).sort(),
        vec![
            Adjacency::Indirect(System::CH, System::AT, System::SLO),
            Adjacency::Indirect(System::CH, System::IT, System::SLO),
        ]
        .sort()
    );
}
