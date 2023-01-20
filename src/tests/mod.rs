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
            pt_mode: Some("bus".to_string()),
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
fn test_trip_parse_ok() {
    let trip_result = OjpDoc::new(
        r#"<?xml version="1.0" encoding="UTF-8"?>
    <siri:OJP
      xmlns:siri="http://www.siri.org.uk/siri"
      xmlns:ojp="http://www.vdv.de/ojp" version="1.0">
      <siri:OJPResponse>
        <siri:ServiceDelivery>
          <siri:ResponseTimestamp>2022-12-12T08:54:17Z</siri:ResponseTimestamp>
          <siri:ProducerRef>EFAController10.5.17.10-OJP-EFA01-P</siri:ProducerRef>
          <siri:Status>true</siri:Status>
          <ojp:OJPTripDelivery>
            <siri:ResponseTimestamp>2022-12-12T08:54:17Z</siri:ResponseTimestamp>
            <siri:Status>true</siri:Status>
            <ojp:CalcTime>406</ojp:CalcTime>
            <ojp:TripResponseContext>
              <ojp:Places>
                <ojp:Location>
                  <ojp:StopPoint>
                    <siri:StopPointRef>8507000</siri:StopPointRef>
                    <ojp:StopPointName>
                      <ojp:Text xml:lang="de">Bern</ojp:Text>
                    </ojp:StopPointName>
                    <ojp:PrivateCode>
                      <ojp:System>EFA</ojp:System>
                      <ojp:Value>111055:0:7</ojp:Value>
                    </ojp:PrivateCode>
                    <ojp:TopographicPlaceRef>23006351:1</ojp:TopographicPlaceRef>
                  </ojp:StopPoint>
                  <ojp:LocationName>
                    <ojp:Text xml:lang="de">Bern</ojp:Text>
                  </ojp:LocationName>
                  <ojp:GeoPosition>
                    <siri:Longitude>7.43913</siri:Longitude>
                    <siri:Latitude>46.94883</siri:Latitude>
                  </ojp:GeoPosition>
                </ojp:Location>
                <ojp:Location>
                  <ojp:TopographicPlace>
                    <ojp:TopographicPlaceCode>23006351:1</ojp:TopographicPlaceCode>
                    <ojp:TopographicPlaceName>
                      <ojp:Text xml:lang="de">Bern</ojp:Text>
                    </ojp:TopographicPlaceName>
                  </ojp:TopographicPlace>
                  <ojp:LocationName>
                    <ojp:Text xml:lang="de">Bern</ojp:Text>
                  </ojp:LocationName>
                  <ojp:GeoPosition>
                    <siri:Longitude>7.43913</siri:Longitude>
                    <siri:Latitude>46.94883</siri:Latitude>
                  </ojp:GeoPosition>
                </ojp:Location>
                <ojp:Location>
                  <ojp:StopPoint>
                    <siri:StopPointRef>8507100</siri:StopPointRef>
                    <ojp:StopPointName>
                      <ojp:Text xml:lang="de">Thun</ojp:Text>
                    </ojp:StopPointName>
                    <ojp:PrivateCode>
                      <ojp:System>EFA</ojp:System>
                      <ojp:Value>111104:0:1</ojp:Value>
                    </ojp:PrivateCode>
                    <ojp:TopographicPlaceRef>23006942:9</ojp:TopographicPlaceRef>
                  </ojp:StopPoint>
                  <ojp:LocationName>
                    <ojp:Text xml:lang="de">Thun</ojp:Text>
                  </ojp:LocationName>
                  <ojp:GeoPosition>
                    <siri:Longitude>7.62961</siri:Longitude>
                    <siri:Latitude>46.75485</siri:Latitude>
                  </ojp:GeoPosition>
                </ojp:Location>
                <ojp:Location>
                  <ojp:TopographicPlace>
                    <ojp:TopographicPlaceCode>23006942:9</ojp:TopographicPlaceCode>
                    <ojp:TopographicPlaceName>
                      <ojp:Text xml:lang="de">Thun</ojp:Text>
                    </ojp:TopographicPlaceName>
                  </ojp:TopographicPlace>
                  <ojp:LocationName>
                    <ojp:Text xml:lang="de">Thun</ojp:Text>
                  </ojp:LocationName>
                  <ojp:GeoPosition>
                    <siri:Longitude>7.62961</siri:Longitude>
                    <siri:Latitude>46.75485</siri:Latitude>
                  </ojp:GeoPosition>
                </ojp:Location>
                <ojp:Location>
                  <ojp:StopPoint>
                    <siri:StopPointRef>8507483</siri:StopPointRef>
                    <ojp:StopPointName>
                      <ojp:Text xml:lang="de">Spiez</ojp:Text>
                    </ojp:StopPointName>
                    <ojp:PrivateCode>
                      <ojp:System>EFA</ojp:System>
                      <ojp:Value>111340:0:1</ojp:Value>
                    </ojp:PrivateCode>
                    <ojp:TopographicPlaceRef>23006768:5</ojp:TopographicPlaceRef>
                  </ojp:StopPoint>
                  <ojp:LocationName>
                    <ojp:Text xml:lang="de">Spiez</ojp:Text>
                  </ojp:LocationName>
                  <ojp:GeoPosition>
                    <siri:Longitude>7.68010</siri:Longitude>
                    <siri:Latitude>46.68639</siri:Latitude>
                  </ojp:GeoPosition>
                </ojp:Location>
                <ojp:Location>
                  <ojp:TopographicPlace>
                    <ojp:TopographicPlaceCode>23006768:5</ojp:TopographicPlaceCode>
                    <ojp:TopographicPlaceName>
                      <ojp:Text xml:lang="de">Spiez</ojp:Text>
                    </ojp:TopographicPlaceName>
                  </ojp:TopographicPlace>
                  <ojp:LocationName>
                    <ojp:Text xml:lang="de">Spiez</ojp:Text>
                  </ojp:LocationName>
                  <ojp:GeoPosition>
                    <siri:Longitude>7.68010</siri:Longitude>
                    <siri:Latitude>46.68639</siri:Latitude>
                  </ojp:GeoPosition>
                </ojp:Location>
                <ojp:Location>
                  <ojp:StopPoint>
                    <siri:StopPointRef>8507493</siri:StopPointRef>
                    <ojp:StopPointName>
                      <ojp:Text xml:lang="de">Interlaken West</ojp:Text>
                    </ojp:StopPointName>
                    <ojp:PrivateCode>
                      <ojp:System>EFA</ojp:System>
                      <ojp:Value>111344:0:1</ojp:Value>
                    </ojp:PrivateCode>
                    <ojp:TopographicPlaceRef>23006581:1</ojp:TopographicPlaceRef>
                  </ojp:StopPoint>
                  <ojp:LocationName>
                    <ojp:Text xml:lang="de">Interlaken West</ojp:Text>
                  </ojp:LocationName>
                  <ojp:GeoPosition>
                    <siri:Longitude>7.85146</siri:Longitude>
                    <siri:Latitude>46.68263</siri:Latitude>
                  </ojp:GeoPosition>
                </ojp:Location>
                <ojp:Location>
                  <ojp:TopographicPlace>
                    <ojp:TopographicPlaceCode>23006581:1</ojp:TopographicPlaceCode>
                    <ojp:TopographicPlaceName>
                      <ojp:Text xml:lang="de">Interlaken</ojp:Text>
                    </ojp:TopographicPlaceName>
                  </ojp:TopographicPlace>
                  <ojp:LocationName>
                    <ojp:Text xml:lang="de">Interlaken</ojp:Text>
                  </ojp:LocationName>
                  <ojp:GeoPosition>
                    <siri:Longitude>7.85146</siri:Longitude>
                    <siri:Latitude>46.68263</siri:Latitude>
                  </ojp:GeoPosition>
                </ojp:Location>
                <ojp:Location>
                  <ojp:StopPoint>
                    <siri:StopPointRef>8507492</siri:StopPointRef>
                    <ojp:StopPointName>
                      <ojp:Text xml:lang="de">Interlaken Ost</ojp:Text>
                    </ojp:StopPointName>
                    <ojp:PrivateCode>
                      <ojp:System>EFA</ojp:System>
                      <ojp:Value>111343:0:7</ojp:Value>
                    </ojp:PrivateCode>
                    <ojp:TopographicPlaceRef>23006581:1</ojp:TopographicPlaceRef>
                  </ojp:StopPoint>
                  <ojp:LocationName>
                    <ojp:Text xml:lang="de">Interlaken Ost</ojp:Text>
                  </ojp:LocationName>
                  <ojp:GeoPosition>
                    <siri:Longitude>7.86900</siri:Longitude>
                    <siri:Latitude>46.69050</siri:Latitude>
                  </ojp:GeoPosition>
                </ojp:Location>
                <ojp:Location>
                  <ojp:StopPoint>
                    <siri:StopPointRef>8507388</siri:StopPointRef>
                    <ojp:StopPointName>
                      <ojp:Text xml:lang="de">Wilderswil</ojp:Text>
                    </ojp:StopPointName>
                    <ojp:PrivateCode>
                      <ojp:System>EFA</ojp:System>
                      <ojp:Value>111278:0:2CD</ojp:Value>
                    </ojp:PrivateCode>
                    <ojp:TopographicPlaceRef>23006594:2</ojp:TopographicPlaceRef>
                  </ojp:StopPoint>
                  <ojp:LocationName>
                    <ojp:Text xml:lang="de">Wilderswil</ojp:Text>
                  </ojp:LocationName>
                  <ojp:GeoPosition>
                    <siri:Longitude>7.86947</siri:Longitude>
                    <siri:Latitude>46.66570</siri:Latitude>
                  </ojp:GeoPosition>
                </ojp:Location>
                <ojp:Location>
                  <ojp:TopographicPlace>
                    <ojp:TopographicPlaceCode>23006594:2</ojp:TopographicPlaceCode>
                    <ojp:TopographicPlaceName>
                      <ojp:Text xml:lang="de">Wilderswil</ojp:Text>
                    </ojp:TopographicPlaceName>
                  </ojp:TopographicPlace>
                  <ojp:LocationName>
                    <ojp:Text xml:lang="de">Wilderswil</ojp:Text>
                  </ojp:LocationName>
                  <ojp:GeoPosition>
                    <siri:Longitude>7.86947</siri:Longitude>
                    <siri:Latitude>46.66570</siri:Latitude>
                  </ojp:GeoPosition>
                </ojp:Location>
                <ojp:Location>
                  <ojp:StopPoint>
                    <siri:StopPointRef>8507389</siri:StopPointRef>
                    <ojp:StopPointName>
                      <ojp:Text xml:lang="de">Zweilütschinen</ojp:Text>
                    </ojp:StopPointName>
                    <ojp:PrivateCode>
                      <ojp:System>EFA</ojp:System>
                      <ojp:Value>111279:0:3CD</ojp:Value>
                    </ojp:PrivateCode>
                    <ojp:TopographicPlaceRef>23006578:2</ojp:TopographicPlaceRef>
                  </ojp:StopPoint>
                  <ojp:LocationName>
                    <ojp:Text xml:lang="de">Zweilütschinen</ojp:Text>
                  </ojp:LocationName>
                  <ojp:GeoPosition>
                    <siri:Longitude>7.89969</siri:Longitude>
                    <siri:Latitude>46.63270</siri:Latitude>
                  </ojp:GeoPosition>
                </ojp:Location>
                <ojp:Location>
                  <ojp:TopographicPlace>
                    <ojp:TopographicPlaceCode>23006578:2</ojp:TopographicPlaceCode>
                    <ojp:TopographicPlaceName>
                      <ojp:Text xml:lang="de">Zweilütschinen</ojp:Text>
                    </ojp:TopographicPlaceName>
                  </ojp:TopographicPlace>
                  <ojp:LocationName>
                    <ojp:Text xml:lang="de">Zweilütschinen</ojp:Text>
                  </ojp:LocationName>
                  <ojp:GeoPosition>
                    <siri:Longitude>7.89969</siri:Longitude>
                    <siri:Latitude>46.63270</siri:Latitude>
                  </ojp:GeoPosition>
                </ojp:Location>
                <ojp:Location>
                  <ojp:StopPoint>
                    <siri:StopPointRef>8507383</siri:StopPointRef>
                    <ojp:StopPointName>
                      <ojp:Text xml:lang="de">Lütschental</ojp:Text>
                    </ojp:StopPointName>
                    <ojp:PrivateCode>
                      <ojp:System>EFA</ojp:System>
                      <ojp:Value>111274:0:2</ojp:Value>
                    </ojp:PrivateCode>
                    <ojp:TopographicPlaceRef>23006586:1</ojp:TopographicPlaceRef>
                  </ojp:StopPoint>
                  <ojp:LocationName>
                    <ojp:Text xml:lang="de">Lütschental</ojp:Text>
                  </ojp:LocationName>
                  <ojp:GeoPosition>
                    <siri:Longitude>7.94899</siri:Longitude>
                    <siri:Latitude>46.63710</siri:Latitude>
                  </ojp:GeoPosition>
                </ojp:Location>
                <ojp:Location>
                  <ojp:TopographicPlace>
                    <ojp:TopographicPlaceCode>23006586:1</ojp:TopographicPlaceCode>
                    <ojp:TopographicPlaceName>
                      <ojp:Text xml:lang="de">Lütschental</ojp:Text>
                    </ojp:TopographicPlaceName>
                  </ojp:TopographicPlace>
                  <ojp:LocationName>
                    <ojp:Text xml:lang="de">Lütschental</ojp:Text>
                  </ojp:LocationName>
                  <ojp:GeoPosition>
                    <siri:Longitude>7.94899</siri:Longitude>
                    <siri:Latitude>46.63710</siri:Latitude>
                  </ojp:GeoPosition>
                </ojp:Location>
                <ojp:Location>
                  <ojp:StopPoint>
                    <siri:StopPointRef>8507382</siri:StopPointRef>
                    <ojp:StopPointName>
                      <ojp:Text xml:lang="de">Burglauenen</ojp:Text>
                    </ojp:StopPointName>
                    <ojp:PrivateCode>
                      <ojp:System>EFA</ojp:System>
                      <ojp:Value>111273:0:2</ojp:Value>
                    </ojp:PrivateCode>
                    <ojp:TopographicPlaceRef>23006576:1</ojp:TopographicPlaceRef>
                  </ojp:StopPoint>
                  <ojp:LocationName>
                    <ojp:Text xml:lang="de">Burglauenen</ojp:Text>
                  </ojp:LocationName>
                  <ojp:GeoPosition>
                    <siri:Longitude>7.97533</siri:Longitude>
                    <siri:Latitude>46.63591</siri:Latitude>
                  </ojp:GeoPosition>
                </ojp:Location>
                <ojp:Location>
                  <ojp:TopographicPlace>
                    <ojp:TopographicPlaceCode>23006576:1</ojp:TopographicPlaceCode>
                    <ojp:TopographicPlaceName>
                      <ojp:Text xml:lang="de">Burglauenen</ojp:Text>
                    </ojp:TopographicPlaceName>
                  </ojp:TopographicPlace>
                  <ojp:LocationName>
                    <ojp:Text xml:lang="de">Burglauenen</ojp:Text>
                  </ojp:LocationName>
                  <ojp:GeoPosition>
                    <siri:Longitude>7.97533</siri:Longitude>
                    <siri:Latitude>46.63591</siri:Latitude>
                  </ojp:GeoPosition>
                </ojp:Location>
                <ojp:Location>
                  <ojp:StopPoint>
                    <siri:StopPointRef>8507381</siri:StopPointRef>
                    <ojp:StopPointName>
                      <ojp:Text xml:lang="de">Schwendi bei Grindelwald</ojp:Text>
                    </ojp:StopPointName>
                    <ojp:PrivateCode>
                      <ojp:System>EFA</ojp:System>
                      <ojp:Value>111272:0:2</ojp:Value>
                    </ojp:PrivateCode>
                    <ojp:TopographicPlaceRef>23006576:2</ojp:TopographicPlaceRef>
                  </ojp:StopPoint>
                  <ojp:LocationName>
                    <ojp:Text xml:lang="de">Schwendi bei Grindelwald</ojp:Text>
                  </ojp:LocationName>
                  <ojp:GeoPosition>
                    <siri:Longitude>8.00307</siri:Longitude>
                    <siri:Latitude>46.63047</siri:Latitude>
                  </ojp:GeoPosition>
                </ojp:Location>
                <ojp:Location>
                  <ojp:TopographicPlace>
                    <ojp:TopographicPlaceCode>23006576:2</ojp:TopographicPlaceCode>
                    <ojp:TopographicPlaceName>
                      <ojp:Text xml:lang="de">Grindelwald</ojp:Text>
                    </ojp:TopographicPlaceName>
                  </ojp:TopographicPlace>
                  <ojp:LocationName>
                    <ojp:Text xml:lang="de">Grindelwald</ojp:Text>
                  </ojp:LocationName>
                  <ojp:GeoPosition>
                    <siri:Longitude>8.00307</siri:Longitude>
                    <siri:Latitude>46.63047</siri:Latitude>
                  </ojp:GeoPosition>
                </ojp:Location>
                <ojp:Location>
                  <ojp:StopPlace>
                    <ojp:StopPlaceRef>8505226</ojp:StopPlaceRef>
                    <ojp:StopPlaceName>
                      <ojp:Text xml:lang="de">Grindelwald Terminal</ojp:Text>
                    </ojp:StopPlaceName>
                    <ojp:PrivateCode>
                      <ojp:System>EFA</ojp:System>
                      <ojp:Value>109869</ojp:Value>
                    </ojp:PrivateCode>
                    <ojp:TopographicPlaceRef>23006576:2</ojp:TopographicPlaceRef>
                  </ojp:StopPlace>
                  <ojp:LocationName>
                    <ojp:Text xml:lang="de">Grindelwald Terminal</ojp:Text>
                  </ojp:LocationName>
                  <ojp:GeoPosition>
                    <siri:Longitude>8.01710</siri:Longitude>
                    <siri:Latitude>46.62551</siri:Latitude>
                  </ojp:GeoPosition>
                </ojp:Location>
                <ojp:Location>
                  <ojp:StopPoint>
                    <siri:StopPointRef>8507380</siri:StopPointRef>
                    <ojp:StopPointName>
                      <ojp:Text xml:lang="de">Grindelwald</ojp:Text>
                    </ojp:StopPointName>
                    <ojp:PrivateCode>
                      <ojp:System>EFA</ojp:System>
                      <ojp:Value>111271:0:1</ojp:Value>
                    </ojp:PrivateCode>
                    <ojp:TopographicPlaceRef>23006576:2</ojp:TopographicPlaceRef>
                  </ojp:StopPoint>
                  <ojp:LocationName>
                    <ojp:Text xml:lang="de">Grindelwald</ojp:Text>
                  </ojp:LocationName>
                  <ojp:GeoPosition>
                    <siri:Longitude>8.03331</siri:Longitude>
                    <siri:Latitude>46.62436</siri:Latitude>
                  </ojp:GeoPosition>
                </ojp:Location>
                <ojp:Location>
                  <ojp:StopPlace>
                    <ojp:StopPlaceRef>8501609</ojp:StopPlaceRef>
                    <ojp:StopPlaceName>
                      <ojp:Text xml:lang="de">Brig</ojp:Text>
                    </ojp:StopPlaceName>
                    <ojp:PrivateCode>
                      <ojp:System>EFA</ojp:System>
                      <ojp:Value>107458</ojp:Value>
                    </ojp:PrivateCode>
                    <ojp:TopographicPlaceRef>23024002:1</ojp:TopographicPlaceRef>
                  </ojp:StopPlace>
                  <ojp:LocationName>
                    <ojp:Text xml:lang="de">Brig</ojp:Text>
                  </ojp:LocationName>
                  <ojp:GeoPosition>
                    <siri:Longitude>7.98810</siri:Longitude>
                    <siri:Latitude>46.31943</siri:Latitude>
                  </ojp:GeoPosition>
                </ojp:Location>
                <ojp:Location>
                  <ojp:TopographicPlace>
                    <ojp:TopographicPlaceCode>23024002:1</ojp:TopographicPlaceCode>
                    <ojp:TopographicPlaceName>
                      <ojp:Text xml:lang="de">Brig</ojp:Text>
                    </ojp:TopographicPlaceName>
                  </ojp:TopographicPlace>
                  <ojp:LocationName>
                    <ojp:Text xml:lang="de">Brig</ojp:Text>
                  </ojp:LocationName>
                  <ojp:GeoPosition>
                    <siri:Longitude>7.98810</siri:Longitude>
                    <siri:Latitude>46.31943</siri:Latitude>
                  </ojp:GeoPosition>
                </ojp:Location>
              </ojp:Places>
            </ojp:TripResponseContext>
            <ojp:TripResult>
              <ojp:ResultId>ID-C3A534D6-BB00-4D13-B489-893663AD6034</ojp:ResultId>
              <ojp:Trip>
                <ojp:TripId>ID-C3A534D6-BB00-4D13-B489-893663AD6034</ojp:TripId>
                <ojp:Duration>PT1H35M</ojp:Duration>
                <ojp:StartTime>2022-12-12T15:34:00Z</ojp:StartTime>
                <ojp:EndTime>2022-12-12T17:09:00Z</ojp:EndTime>
                <ojp:Transfers>1</ojp:Transfers>
                <ojp:Distance>0</ojp:Distance>
                <ojp:TripLeg>
                  <ojp:LegId>1</ojp:LegId>
                  <ojp:TimedLeg>
                    <ojp:LegBoard>
                      <siri:StopPointRef>8507000</siri:StopPointRef>
                      <ojp:StopPointName>
                        <ojp:Text xml:lang="de">Bern</ojp:Text>
                      </ojp:StopPointName>
                      <ojp:PlannedQuay>
                        <ojp:Text xml:lang="de">7</ojp:Text>
                      </ojp:PlannedQuay>
                      <ojp:ServiceDeparture>
                        <ojp:TimetabledTime>2022-12-12T15:34:00Z</ojp:TimetabledTime>
                      </ojp:ServiceDeparture>
                      <ojp:Order>1</ojp:Order>
                    </ojp:LegBoard>
                    <ojp:LegIntermediates>
                      <siri:StopPointRef>8507100</siri:StopPointRef>
                      <ojp:StopPointName>
                        <ojp:Text xml:lang="de">Thun</ojp:Text>
                      </ojp:StopPointName>
                      <ojp:PlannedQuay>
                        <ojp:Text xml:lang="de">1</ojp:Text>
                      </ojp:PlannedQuay>
                      <ojp:ServiceArrival>
                        <ojp:TimetabledTime>2022-12-12T15:52:00Z</ojp:TimetabledTime>
                      </ojp:ServiceArrival>
                      <ojp:ServiceDeparture>
                        <ojp:TimetabledTime>2022-12-12T15:54:00Z</ojp:TimetabledTime>
                      </ojp:ServiceDeparture>
                      <ojp:Order>2</ojp:Order>
                    </ojp:LegIntermediates>
                    <ojp:LegIntermediates>
                      <siri:StopPointRef>8507483</siri:StopPointRef>
                      <ojp:StopPointName>
                        <ojp:Text xml:lang="de">Spiez</ojp:Text>
                      </ojp:StopPointName>
                      <ojp:PlannedQuay>
                        <ojp:Text xml:lang="de">1</ojp:Text>
                      </ojp:PlannedQuay>
                      <ojp:ServiceArrival>
                        <ojp:TimetabledTime>2022-12-12T16:03:00Z</ojp:TimetabledTime>
                      </ojp:ServiceArrival>
                      <ojp:ServiceDeparture>
                        <ojp:TimetabledTime>2022-12-12T16:04:00Z</ojp:TimetabledTime>
                      </ojp:ServiceDeparture>
                      <ojp:Order>3</ojp:Order>
                    </ojp:LegIntermediates>
                    <ojp:LegIntermediates>
                      <siri:StopPointRef>8507493</siri:StopPointRef>
                      <ojp:StopPointName>
                        <ojp:Text xml:lang="de">Interlaken West</ojp:Text>
                      </ojp:StopPointName>
                      <ojp:PlannedQuay>
                        <ojp:Text xml:lang="de">1</ojp:Text>
                      </ojp:PlannedQuay>
                      <ojp:ServiceArrival>
                        <ojp:TimetabledTime>2022-12-12T16:22:00Z</ojp:TimetabledTime>
                      </ojp:ServiceArrival>
                      <ojp:ServiceDeparture>
                        <ojp:TimetabledTime>2022-12-12T16:23:00Z</ojp:TimetabledTime>
                      </ojp:ServiceDeparture>
                      <ojp:Order>4</ojp:Order>
                    </ojp:LegIntermediates>
                    <ojp:LegAlight>
                      <siri:StopPointRef>8507492</siri:StopPointRef>
                      <ojp:StopPointName>
                        <ojp:Text xml:lang="de">Interlaken Ost</ojp:Text>
                      </ojp:StopPointName>
                      <ojp:PlannedQuay>
                        <ojp:Text xml:lang="de">7</ojp:Text>
                      </ojp:PlannedQuay>
                      <ojp:ServiceArrival>
                        <ojp:TimetabledTime>2022-12-12T16:28:00Z</ojp:TimetabledTime>
                      </ojp:ServiceArrival>
                      <ojp:Order>5</ojp:Order>
                    </ojp:LegAlight>
                    <ojp:Service>
                      <ojp:OperatingDayRef>2022-12-12</ojp:OperatingDayRef>
                      <ojp:JourneyRef>ojp:91061:A:H:j23:210:1077</ojp:JourneyRef>
                      <siri:LineRef>ojp:91061:A</siri:LineRef>
                      <siri:DirectionRef>H</siri:DirectionRef>
                      <ojp:Mode>
                        <ojp:PtMode>rail</ojp:PtMode>
                        <siri:RailSubmode>interRegionalRailService</siri:RailSubmode>
                        <ojp:Name>
                          <ojp:Text xml:lang="de">Zug</ojp:Text>
                        </ojp:Name>
                        <ojp:ShortName>
                          <ojp:Text xml:lang="de">IC</ojp:Text>
                        </ojp:ShortName>
                      </ojp:Mode>
                      <ojp:PublishedLineName>
                        <ojp:Text xml:lang="de">IC61</ojp:Text>
                      </ojp:PublishedLineName>
                      <ojp:OperatorRef>ojp:11</ojp:OperatorRef>
                      <ojp:Attribute>
                        <ojp:Text>
                          <ojp:Text xml:lang="de">Businesszone in 1. Klasse</ojp:Text>
                        </ojp:Text>
                        <ojp:Code>A__BZ</ojp:Code>
                        <siri:FareClassFacility>firstClass</siri:FareClassFacility>
                        <siri:PassengerCommsFacility>businessServices</siri:PassengerCommsFacility>
                      </ojp:Attribute>
                      <ojp:Attribute>
                        <ojp:Text>
                          <ojp:Text xml:lang="de">Ruhezone in 1. Klasse</ojp:Text>
                        </ojp:Text>
                        <ojp:Code>A__RZ</ojp:Code>
                        <siri:NuisanceFacility>mobilePhoneFreeZone</siri:NuisanceFacility>
                      </ojp:Attribute>
                      <ojp:Attribute>
                        <ojp:Text>
                          <ojp:Text xml:lang="de">Gratis-Internet mit der App SBB FreeSurf</ojp:Text>
                        </ojp:Text>
                        <ojp:Code>A__FS</ojp:Code>
                      </ojp:Attribute>
                      <ojp:Attribute>
                        <ojp:Text>
                          <ojp:Text xml:lang="de">Familienwagen mit Spielplatz</ojp:Text>
                        </ojp:Text>
                        <ojp:Code>A__FA</ojp:Code>
                      </ojp:Attribute>
                      <ojp:Attribute>
                        <ojp:Text>
                          <ojp:Text xml:lang="de">Platzreservierung möglich</ojp:Text>
                        </ojp:Text>
                        <ojp:Code>A___R</ojp:Code>
                      </ojp:Attribute>
                      <ojp:Attribute>
                        <ojp:Text>
                          <ojp:Text xml:lang="de">Restaurant</ojp:Text>
                        </ojp:Text>
                        <ojp:Code>A__WR</ojp:Code>
                        <siri:RefreshmentFacility>restaurantService</siri:RefreshmentFacility>
                      </ojp:Attribute>
                      <ojp:Attribute>
                        <ojp:Text>
                          <ojp:Text xml:lang="de">Aussteigeseite: Rechts</ojp:Text>
                        </ojp:Text>
                        <ojp:Code>ojp91061AH_InfoCall210_111055_1</ojp:Code>
                      </ojp:Attribute>
                      <ojp:Attribute>
                        <ojp:Text>
                          <ojp:Text xml:lang="de">Aussteigeseite: Links</ojp:Text>
                        </ojp:Text>
                        <ojp:Code>ojp91061AH_InfoCall210_111343_1</ojp:Code>
                      </ojp:Attribute>
                      <ojp:DestinationStopPointRef>8507492</ojp:DestinationStopPointRef>
                      <ojp:DestinationText>
                        <ojp:Text xml:lang="de">Interlaken Ost</ojp:Text>
                      </ojp:DestinationText>
                    </ojp:Service>
                    <ojp:Extension>
                      <ojp:TransportTypeName>
                        <ojp:Text xml:lang="de">InterCity</ojp:Text>
                      </ojp:TransportTypeName>
                      <ojp:PublishedJourneyNumber>
                        <ojp:Text xml:lang="de">1077</ojp:Text>
                      </ojp:PublishedJourneyNumber>
                      <ojp:OperatorName>
                        <ojp:Text xml:lang="de">Schweizerische Bundesbahnen SBB</ojp:Text>
                      </ojp:OperatorName>
                    </ojp:Extension>
                  </ojp:TimedLeg>
                </ojp:TripLeg>
                <ojp:TripLeg>
                  <ojp:LegId>2</ojp:LegId>
                  <ojp:TransferLeg>
                    <ojp:TransferMode>walk</ojp:TransferMode>
                    <ojp:LegStart>
                      <siri:StopPointRef>8507492</siri:StopPointRef>
                      <ojp:LocationName>
                        <ojp:Text xml:lang="de">Interlaken Ost</ojp:Text>
                      </ojp:LocationName>
                    </ojp:LegStart>
                    <ojp:LegEnd>
                      <siri:StopPointRef>8507492</siri:StopPointRef>
                      <ojp:LocationName>
                        <ojp:Text xml:lang="de">Interlaken Ost</ojp:Text>
                      </ojp:LocationName>
                    </ojp:LegEnd>
                    <ojp:TimeWindowStart>2022-12-12T16:28:00Z</ojp:TimeWindowStart>
                    <ojp:TimeWindowEnd>2022-12-12T16:35:00Z</ojp:TimeWindowEnd>
                    <ojp:Duration>PT7M</ojp:Duration>
                    <ojp:WalkDuration>PT5M</ojp:WalkDuration>
                    <ojp:BufferTime>PT2M</ojp:BufferTime>
                  </ojp:TransferLeg>
                </ojp:TripLeg>
                <ojp:TripLeg>
                  <ojp:LegId>3</ojp:LegId>
                  <ojp:TimedLeg>
                    <ojp:LegBoard>
                      <siri:StopPointRef>8507492</siri:StopPointRef>
                      <ojp:StopPointName>
                        <ojp:Text xml:lang="de">Interlaken Ost</ojp:Text>
                      </ojp:StopPointName>
                      <ojp:PlannedQuay>
                        <ojp:Text xml:lang="de">2B</ojp:Text>
                      </ojp:PlannedQuay>
                      <ojp:ServiceDeparture>
                        <ojp:TimetabledTime>2022-12-12T16:35:00Z</ojp:TimetabledTime>
                      </ojp:ServiceDeparture>
                      <ojp:Order>1</ojp:Order>
                    </ojp:LegBoard>
                    <ojp:LegIntermediates>
                      <siri:StopPointRef>8507388</siri:StopPointRef>
                      <ojp:StopPointName>
                        <ojp:Text xml:lang="de">Wilderswil</ojp:Text>
                      </ojp:StopPointName>
                      <ojp:PlannedQuay>
                        <ojp:Text xml:lang="de">2CD</ojp:Text>
                      </ojp:PlannedQuay>
                      <ojp:ServiceArrival>
                        <ojp:TimetabledTime>2022-12-12T16:39:00Z</ojp:TimetabledTime>
                      </ojp:ServiceArrival>
                      <ojp:ServiceDeparture>
                        <ojp:TimetabledTime>2022-12-12T16:40:00Z</ojp:TimetabledTime>
                      </ojp:ServiceDeparture>
                      <ojp:Order>2</ojp:Order>
                    </ojp:LegIntermediates>
                    <ojp:LegIntermediates>
                      <siri:StopPointRef>8507389</siri:StopPointRef>
                      <ojp:StopPointName>
                        <ojp:Text xml:lang="de">Zweilütschinen</ojp:Text>
                      </ojp:StopPointName>
                      <ojp:PlannedQuay>
                        <ojp:Text xml:lang="de">3CD</ojp:Text>
                      </ojp:PlannedQuay>
                      <ojp:ServiceArrival>
                        <ojp:TimetabledTime>2022-12-12T16:45:00Z</ojp:TimetabledTime>
                      </ojp:ServiceArrival>
                      <ojp:ServiceDeparture>
                        <ojp:TimetabledTime>2022-12-12T16:47:00Z</ojp:TimetabledTime>
                      </ojp:ServiceDeparture>
                      <ojp:Order>3</ojp:Order>
                    </ojp:LegIntermediates>
                    <ojp:LegIntermediates>
                      <siri:StopPointRef>8507383</siri:StopPointRef>
                      <ojp:StopPointName>
                        <ojp:Text xml:lang="de">Lütschental</ojp:Text>
                      </ojp:StopPointName>
                      <ojp:PlannedQuay>
                        <ojp:Text xml:lang="de">2</ojp:Text>
                      </ojp:PlannedQuay>
                      <ojp:ServiceArrival>
                        <ojp:TimetabledTime>2022-12-12T16:52:00Z</ojp:TimetabledTime>
                      </ojp:ServiceArrival>
                      <ojp:ServiceDeparture>
                        <ojp:TimetabledTime>2022-12-12T16:52:00Z</ojp:TimetabledTime>
                      </ojp:ServiceDeparture>
                      <ojp:Order>4</ojp:Order>
                    </ojp:LegIntermediates>
                    <ojp:LegIntermediates>
                      <siri:StopPointRef>8507382</siri:StopPointRef>
                      <ojp:StopPointName>
                        <ojp:Text xml:lang="de">Burglauenen</ojp:Text>
                      </ojp:StopPointName>
                      <ojp:PlannedQuay>
                        <ojp:Text xml:lang="de">2</ojp:Text>
                      </ojp:PlannedQuay>
                      <ojp:ServiceArrival>
                        <ojp:TimetabledTime>2022-12-12T16:57:00Z</ojp:TimetabledTime>
                      </ojp:ServiceArrival>
                      <ojp:ServiceDeparture>
                        <ojp:TimetabledTime>2022-12-12T16:58:00Z</ojp:TimetabledTime>
                      </ojp:ServiceDeparture>
                      <ojp:Order>5</ojp:Order>
                    </ojp:LegIntermediates>
                    <ojp:LegIntermediates>
                      <siri:StopPointRef>8507381</siri:StopPointRef>
                      <ojp:StopPointName>
                        <ojp:Text xml:lang="de">Schwendi bei Grindelwald</ojp:Text>
                      </ojp:StopPointName>
                      <ojp:PlannedQuay>
                        <ojp:Text xml:lang="de">2</ojp:Text>
                      </ojp:PlannedQuay>
                      <ojp:ServiceArrival>
                        <ojp:TimetabledTime>2022-12-12T17:01:00Z</ojp:TimetabledTime>
                      </ojp:ServiceArrival>
                      <ojp:ServiceDeparture>
                        <ojp:TimetabledTime>2022-12-12T17:01:00Z</ojp:TimetabledTime>
                      </ojp:ServiceDeparture>
                      <ojp:Order>6</ojp:Order>
                    </ojp:LegIntermediates>
                    <ojp:LegIntermediates>
                      <siri:StopPointRef>8505226</siri:StopPointRef>
                      <ojp:StopPointName>
                        <ojp:Text xml:lang="de">Grindelwald Terminal</ojp:Text>
                      </ojp:StopPointName>
                      <ojp:ServiceArrival>
                        <ojp:TimetabledTime>2022-12-12T17:04:00Z</ojp:TimetabledTime>
                      </ojp:ServiceArrival>
                      <ojp:ServiceDeparture>
                        <ojp:TimetabledTime>2022-12-12T17:05:00Z</ojp:TimetabledTime>
                      </ojp:ServiceDeparture>
                      <ojp:Order>7</ojp:Order>
                    </ojp:LegIntermediates>
                    <ojp:LegAlight>
                      <siri:StopPointRef>8507380</siri:StopPointRef>
                      <ojp:StopPointName>
                        <ojp:Text xml:lang="de">Grindelwald</ojp:Text>
                      </ojp:StopPointName>
                      <ojp:PlannedQuay>
                        <ojp:Text xml:lang="de">1</ojp:Text>
                      </ojp:PlannedQuay>
                      <ojp:ServiceArrival>
                        <ojp:TimetabledTime>2022-12-12T17:09:00Z</ojp:TimetabledTime>
                      </ojp:ServiceArrival>
                      <ojp:Order>8</ojp:Order>
                    </ojp:LegAlight>
                    <ojp:Service>
                      <ojp:OperatingDayRef>2022-12-12</ojp:OperatingDayRef>
                      <ojp:JourneyRef>ojp:91061::H:j23:127:281</ojp:JourneyRef>
                      <siri:LineRef>ojp:91061:</siri:LineRef>
                      <siri:DirectionRef>H</siri:DirectionRef>
                      <ojp:Mode>
                        <ojp:PtMode>rail</ojp:PtMode>
                        <siri:RailSubmode>regionalRail</siri:RailSubmode>
                        <ojp:Name>
                          <ojp:Text xml:lang="de">Zug</ojp:Text>
                        </ojp:Name>
                        <ojp:ShortName>
                          <ojp:Text xml:lang="de">R</ojp:Text>
                        </ojp:ShortName>
                      </ojp:Mode>
                      <ojp:PublishedLineName>
                        <ojp:Text xml:lang="de">R61</ojp:Text>
                      </ojp:PublishedLineName>
                      <ojp:OperatorRef>ojp:35</ojp:OperatorRef>
                      <ojp:DestinationStopPointRef>8507380</ojp:DestinationStopPointRef>
                      <ojp:DestinationText>
                        <ojp:Text xml:lang="de">Grindelwald</ojp:Text>
                      </ojp:DestinationText>
                    </ojp:Service>
                    <ojp:Extension>
                      <ojp:TransportTypeName>
                        <ojp:Text xml:lang="de">Regio</ojp:Text>
                      </ojp:TransportTypeName>
                      <ojp:PublishedJourneyNumber>
                        <ojp:Text xml:lang="de">281</ojp:Text>
                      </ojp:PublishedJourneyNumber>
                      <ojp:OperatorName>
                        <ojp:Text xml:lang="de">Berner Oberland-Bahnen</ojp:Text>
                      </ojp:OperatorName>
                    </ojp:Extension>
                  </ojp:TimedLeg>
                </ojp:TripLeg>
              </ojp:Trip>
            </ojp:TripResult>
            <ojp:TripResult>
              <ojp:ResultId>ID-E8EF7ABA-1C1F-4933-935C-CE733A157002</ojp:ResultId>
              <ojp:Trip>
                <ojp:TripId>ID-E8EF7ABA-1C1F-4933-935C-CE733A157002</ojp:TripId>
                <ojp:Duration>PT1H35M</ojp:Duration>
                <ojp:StartTime>2022-12-12T16:04:00Z</ojp:StartTime>
                <ojp:EndTime>2022-12-12T17:39:00Z</ojp:EndTime>
                <ojp:Transfers>1</ojp:Transfers>
                <ojp:Distance>0</ojp:Distance>
                <ojp:TripLeg>
                  <ojp:LegId>1</ojp:LegId>
                  <ojp:TimedLeg>
                    <ojp:LegBoard>
                      <siri:StopPointRef>8507000</siri:StopPointRef>
                      <ojp:StopPointName>
                        <ojp:Text xml:lang="de">Bern</ojp:Text>
                      </ojp:StopPointName>
                      <ojp:PlannedQuay>
                        <ojp:Text xml:lang="de">8</ojp:Text>
                      </ojp:PlannedQuay>
                      <ojp:ServiceDeparture>
                        <ojp:TimetabledTime>2022-12-12T16:04:00Z</ojp:TimetabledTime>
                      </ojp:ServiceDeparture>
                      <ojp:Order>1</ojp:Order>
                    </ojp:LegBoard>
                    <ojp:LegIntermediates>
                      <siri:StopPointRef>8507100</siri:StopPointRef>
                      <ojp:StopPointName>
                        <ojp:Text xml:lang="de">Thun</ojp:Text>
                      </ojp:StopPointName>
                      <ojp:PlannedQuay>
                        <ojp:Text xml:lang="de">1</ojp:Text>
                      </ojp:PlannedQuay>
                      <ojp:ServiceArrival>
                        <ojp:TimetabledTime>2022-12-12T16:23:00Z</ojp:TimetabledTime>
                      </ojp:ServiceArrival>
                      <ojp:ServiceDeparture>
                        <ojp:TimetabledTime>2022-12-12T16:24:00Z</ojp:TimetabledTime>
                      </ojp:ServiceDeparture>
                      <ojp:Order>2</ojp:Order>
                    </ojp:LegIntermediates>
                    <ojp:LegIntermediates>
                      <siri:StopPointRef>8507483</siri:StopPointRef>
                      <ojp:StopPointName>
                        <ojp:Text xml:lang="de">Spiez</ojp:Text>
                      </ojp:StopPointName>
                      <ojp:PlannedQuay>
                        <ojp:Text xml:lang="de">1</ojp:Text>
                      </ojp:PlannedQuay>
                      <ojp:ServiceArrival>
                        <ojp:TimetabledTime>2022-12-12T16:33:00Z</ojp:TimetabledTime>
                      </ojp:ServiceArrival>
                      <ojp:ServiceDeparture>
                        <ojp:TimetabledTime>2022-12-12T16:34:00Z</ojp:TimetabledTime>
                      </ojp:ServiceDeparture>
                      <ojp:Order>3</ojp:Order>
                    </ojp:LegIntermediates>
                    <ojp:LegIntermediates>
                      <siri:StopPointRef>8507493</siri:StopPointRef>
                      <ojp:StopPointName>
                        <ojp:Text xml:lang="de">Interlaken West</ojp:Text>
                      </ojp:StopPointName>
                      <ojp:PlannedQuay>
                        <ojp:Text xml:lang="de">1</ojp:Text>
                      </ojp:PlannedQuay>
                      <ojp:ServiceArrival>
                        <ojp:TimetabledTime>2022-12-12T16:51:00Z</ojp:TimetabledTime>
                      </ojp:ServiceArrival>
                      <ojp:ServiceDeparture>
                        <ojp:TimetabledTime>2022-12-12T16:53:00Z</ojp:TimetabledTime>
                      </ojp:ServiceDeparture>
                      <ojp:Order>4</ojp:Order>
                    </ojp:LegIntermediates>
                    <ojp:LegAlight>
                      <siri:StopPointRef>8507492</siri:StopPointRef>
                      <ojp:StopPointName>
                        <ojp:Text xml:lang="de">Interlaken Ost</ojp:Text>
                      </ojp:StopPointName>
                      <ojp:PlannedQuay>
                        <ojp:Text xml:lang="de">7</ojp:Text>
                      </ojp:PlannedQuay>
                      <ojp:ServiceArrival>
                        <ojp:TimetabledTime>2022-12-12T16:58:00Z</ojp:TimetabledTime>
                      </ojp:ServiceArrival>
                      <ojp:Order>5</ojp:Order>
                    </ojp:LegAlight>
                    <ojp:Service>
                      <ojp:OperatingDayRef>2022-12-12</ojp:OperatingDayRef>
                      <ojp:JourneyRef>ojp:91061:A:H:j23:221:977</ojp:JourneyRef>
                      <siri:LineRef>ojp:91061:A</siri:LineRef>
                      <siri:DirectionRef>H</siri:DirectionRef>
                      <ojp:Mode>
                        <ojp:PtMode>rail</ojp:PtMode>
                        <siri:RailSubmode>interRegionalRailService</siri:RailSubmode>
                        <ojp:Name>
                          <ojp:Text xml:lang="de">Zug</ojp:Text>
                        </ojp:Name>
                        <ojp:ShortName>
                          <ojp:Text xml:lang="de">IC</ojp:Text>
                        </ojp:ShortName>
                      </ojp:Mode>
                      <ojp:PublishedLineName>
                        <ojp:Text xml:lang="de">IC61</ojp:Text>
                      </ojp:PublishedLineName>
                      <ojp:OperatorRef>ojp:11</ojp:OperatorRef>
                      <ojp:Attribute>
                        <ojp:Text>
                          <ojp:Text xml:lang="de">Businesszone in 1. Klasse</ojp:Text>
                        </ojp:Text>
                        <ojp:Code>A__BZ</ojp:Code>
                        <siri:FareClassFacility>firstClass</siri:FareClassFacility>
                        <siri:PassengerCommsFacility>businessServices</siri:PassengerCommsFacility>
                      </ojp:Attribute>
                      <ojp:Attribute>
                        <ojp:Text>
                          <ojp:Text xml:lang="de">Ruhezone in 1. Klasse</ojp:Text>
                        </ojp:Text>
                        <ojp:Code>A__RZ</ojp:Code>
                        <siri:NuisanceFacility>mobilePhoneFreeZone</siri:NuisanceFacility>
                      </ojp:Attribute>
                      <ojp:Attribute>
                        <ojp:Text>
                          <ojp:Text xml:lang="de">Gratis-Internet mit der App SBB FreeSurf</ojp:Text>
                        </ojp:Text>
                        <ojp:Code>A__FS</ojp:Code>
                      </ojp:Attribute>
                      <ojp:Attribute>
                        <ojp:Text>
                          <ojp:Text xml:lang="de">Platzreservierung möglich</ojp:Text>
                        </ojp:Text>
                        <ojp:Code>A___R</ojp:Code>
                      </ojp:Attribute>
                      <ojp:Attribute>
                        <ojp:Text>
                          <ojp:Text xml:lang="de">Restaurant</ojp:Text>
                        </ojp:Text>
                        <ojp:Code>A__WR</ojp:Code>
                        <siri:RefreshmentFacility>restaurantService</siri:RefreshmentFacility>
                      </ojp:Attribute>
                      <ojp:Attribute>
                        <ojp:Text>
                          <ojp:Text xml:lang="de">Familienwagen mit Spielplatz</ojp:Text>
                        </ojp:Text>
                        <ojp:Code>A__FA</ojp:Code>
                      </ojp:Attribute>
                      <ojp:Attribute>
                        <ojp:Text>
                          <ojp:Text xml:lang="de">Aussteigeseite: Links</ojp:Text>
                        </ojp:Text>
                        <ojp:Code>ojp91061AH_InfoCall221_111055_1</ojp:Code>
                      </ojp:Attribute>
                      <ojp:DestinationStopPointRef>8507492</ojp:DestinationStopPointRef>
                      <ojp:DestinationText>
                        <ojp:Text xml:lang="de">Interlaken Ost</ojp:Text>
                      </ojp:DestinationText>
                    </ojp:Service>
                    <ojp:Extension>
                      <ojp:TransportTypeName>
                        <ojp:Text xml:lang="de">InterCity</ojp:Text>
                      </ojp:TransportTypeName>
                      <ojp:PublishedJourneyNumber>
                        <ojp:Text xml:lang="de">977</ojp:Text>
                      </ojp:PublishedJourneyNumber>
                      <ojp:OperatorName>
                        <ojp:Text xml:lang="de">Schweizerische Bundesbahnen SBB</ojp:Text>
                      </ojp:OperatorName>
                    </ojp:Extension>
                  </ojp:TimedLeg>
                </ojp:TripLeg>
                <ojp:TripLeg>
                  <ojp:LegId>2</ojp:LegId>
                  <ojp:TransferLeg>
                    <ojp:TransferMode>walk</ojp:TransferMode>
                    <ojp:LegStart>
                      <siri:StopPointRef>8507492</siri:StopPointRef>
                      <ojp:LocationName>
                        <ojp:Text xml:lang="de">Interlaken Ost</ojp:Text>
                      </ojp:LocationName>
                    </ojp:LegStart>
                    <ojp:LegEnd>
                      <siri:StopPointRef>8507492</siri:StopPointRef>
                      <ojp:LocationName>
                        <ojp:Text xml:lang="de">Interlaken Ost</ojp:Text>
                      </ojp:LocationName>
                    </ojp:LegEnd>
                    <ojp:TimeWindowStart>2022-12-12T16:58:00Z</ojp:TimeWindowStart>
                    <ojp:TimeWindowEnd>2022-12-12T17:05:00Z</ojp:TimeWindowEnd>
                    <ojp:Duration>PT7M</ojp:Duration>
                    <ojp:WalkDuration>PT5M</ojp:WalkDuration>
                    <ojp:BufferTime>PT2M</ojp:BufferTime>
                  </ojp:TransferLeg>
                </ojp:TripLeg>
                <ojp:TripLeg>
                  <ojp:LegId>3</ojp:LegId>
                  <ojp:TimedLeg>
                    <ojp:LegBoard>
                      <siri:StopPointRef>8507492</siri:StopPointRef>
                      <ojp:StopPointName>
                        <ojp:Text xml:lang="de">Interlaken Ost</ojp:Text>
                      </ojp:StopPointName>
                      <ojp:PlannedQuay>
                        <ojp:Text xml:lang="de">2B</ojp:Text>
                      </ojp:PlannedQuay>
                      <ojp:ServiceDeparture>
                        <ojp:TimetabledTime>2022-12-12T17:05:00Z</ojp:TimetabledTime>
                      </ojp:ServiceDeparture>
                      <ojp:Order>1</ojp:Order>
                    </ojp:LegBoard>
                    <ojp:LegIntermediates>
                      <siri:StopPointRef>8507388</siri:StopPointRef>
                      <ojp:StopPointName>
                        <ojp:Text xml:lang="de">Wilderswil</ojp:Text>
                      </ojp:StopPointName>
                      <ojp:PlannedQuay>
                        <ojp:Text xml:lang="de">2CD</ojp:Text>
                      </ojp:PlannedQuay>
                      <ojp:ServiceArrival>
                        <ojp:TimetabledTime>2022-12-12T17:09:00Z</ojp:TimetabledTime>
                      </ojp:ServiceArrival>
                      <ojp:ServiceDeparture>
                        <ojp:TimetabledTime>2022-12-12T17:10:00Z</ojp:TimetabledTime>
                      </ojp:ServiceDeparture>
                      <ojp:Order>2</ojp:Order>
                    </ojp:LegIntermediates>
                    <ojp:LegIntermediates>
                      <siri:StopPointRef>8507389</siri:StopPointRef>
                      <ojp:StopPointName>
                        <ojp:Text xml:lang="de">Zweilütschinen</ojp:Text>
                      </ojp:StopPointName>
                      <ojp:PlannedQuay>
                        <ojp:Text xml:lang="de">3CD</ojp:Text>
                      </ojp:PlannedQuay>
                      <ojp:ServiceArrival>
                        <ojp:TimetabledTime>2022-12-12T17:15:00Z</ojp:TimetabledTime>
                      </ojp:ServiceArrival>
                      <ojp:ServiceDeparture>
                        <ojp:TimetabledTime>2022-12-12T17:17:00Z</ojp:TimetabledTime>
                      </ojp:ServiceDeparture>
                      <ojp:Order>3</ojp:Order>
                    </ojp:LegIntermediates>
                    <ojp:LegIntermediates>
                      <siri:StopPointRef>8507383</siri:StopPointRef>
                      <ojp:StopPointName>
                        <ojp:Text xml:lang="de">Lütschental</ojp:Text>
                      </ojp:StopPointName>
                      <ojp:PlannedQuay>
                        <ojp:Text xml:lang="de">2</ojp:Text>
                      </ojp:PlannedQuay>
                      <ojp:ServiceArrival>
                        <ojp:TimetabledTime>2022-12-12T17:22:00Z</ojp:TimetabledTime>
                      </ojp:ServiceArrival>
                      <ojp:ServiceDeparture>
                        <ojp:TimetabledTime>2022-12-12T17:22:00Z</ojp:TimetabledTime>
                      </ojp:ServiceDeparture>
                      <ojp:Order>4</ojp:Order>
                    </ojp:LegIntermediates>
                    <ojp:LegIntermediates>
                      <siri:StopPointRef>8507382</siri:StopPointRef>
                      <ojp:StopPointName>
                        <ojp:Text xml:lang="de">Burglauenen</ojp:Text>
                      </ojp:StopPointName>
                      <ojp:PlannedQuay>
                        <ojp:Text xml:lang="de">2</ojp:Text>
                      </ojp:PlannedQuay>
                      <ojp:ServiceArrival>
                        <ojp:TimetabledTime>2022-12-12T17:27:00Z</ojp:TimetabledTime>
                      </ojp:ServiceArrival>
                      <ojp:ServiceDeparture>
                        <ojp:TimetabledTime>2022-12-12T17:28:00Z</ojp:TimetabledTime>
                      </ojp:ServiceDeparture>
                      <ojp:Order>5</ojp:Order>
                    </ojp:LegIntermediates>
                    <ojp:LegIntermediates>
                      <siri:StopPointRef>8507381</siri:StopPointRef>
                      <ojp:StopPointName>
                        <ojp:Text xml:lang="de">Schwendi bei Grindelwald</ojp:Text>
                      </ojp:StopPointName>
                      <ojp:PlannedQuay>
                        <ojp:Text xml:lang="de">2</ojp:Text>
                      </ojp:PlannedQuay>
                      <ojp:ServiceArrival>
                        <ojp:TimetabledTime>2022-12-12T17:31:00Z</ojp:TimetabledTime>
                      </ojp:ServiceArrival>
                      <ojp:ServiceDeparture>
                        <ojp:TimetabledTime>2022-12-12T17:31:00Z</ojp:TimetabledTime>
                      </ojp:ServiceDeparture>
                      <ojp:Order>6</ojp:Order>
                    </ojp:LegIntermediates>
                    <ojp:LegIntermediates>
                      <siri:StopPointRef>8505226</siri:StopPointRef>
                      <ojp:StopPointName>
                        <ojp:Text xml:lang="de">Grindelwald Terminal</ojp:Text>
                      </ojp:StopPointName>
                      <ojp:ServiceArrival>
                        <ojp:TimetabledTime>2022-12-12T17:34:00Z</ojp:TimetabledTime>
                      </ojp:ServiceArrival>
                      <ojp:ServiceDeparture>
                        <ojp:TimetabledTime>2022-12-12T17:35:00Z</ojp:TimetabledTime>
                      </ojp:ServiceDeparture>
                      <ojp:Order>7</ojp:Order>
                    </ojp:LegIntermediates>
                    <ojp:LegAlight>
                      <siri:StopPointRef>8507380</siri:StopPointRef>
                      <ojp:StopPointName>
                        <ojp:Text xml:lang="de">Grindelwald</ojp:Text>
                      </ojp:StopPointName>
                      <ojp:PlannedQuay>
                        <ojp:Text xml:lang="de">1</ojp:Text>
                      </ojp:PlannedQuay>
                      <ojp:ServiceArrival>
                        <ojp:TimetabledTime>2022-12-12T17:39:00Z</ojp:TimetabledTime>
                      </ojp:ServiceArrival>
                      <ojp:Order>8</ojp:Order>
                    </ojp:LegAlight>
                    <ojp:Service>
                      <ojp:OperatingDayRef>2022-12-12</ojp:OperatingDayRef>
                      <ojp:JourneyRef>ojp:91061::H:j23:128:283</ojp:JourneyRef>
                      <siri:LineRef>ojp:91061:</siri:LineRef>
                      <siri:DirectionRef>H</siri:DirectionRef>
                      <ojp:Mode>
                        <ojp:PtMode>rail</ojp:PtMode>
                        <siri:RailSubmode>regionalRail</siri:RailSubmode>
                        <ojp:Name>
                          <ojp:Text xml:lang="de">Zug</ojp:Text>
                        </ojp:Name>
                        <ojp:ShortName>
                          <ojp:Text xml:lang="de">R</ojp:Text>
                        </ojp:ShortName>
                      </ojp:Mode>
                      <ojp:PublishedLineName>
                        <ojp:Text xml:lang="de">R61</ojp:Text>
                      </ojp:PublishedLineName>
                      <ojp:OperatorRef>ojp:35</ojp:OperatorRef>
                      <ojp:DestinationStopPointRef>8507380</ojp:DestinationStopPointRef>
                      <ojp:DestinationText>
                        <ojp:Text xml:lang="de">Grindelwald</ojp:Text>
                      </ojp:DestinationText>
                    </ojp:Service>
                    <ojp:Extension>
                      <ojp:TransportTypeName>
                        <ojp:Text xml:lang="de">Regio</ojp:Text>
                      </ojp:TransportTypeName>
                      <ojp:PublishedJourneyNumber>
                        <ojp:Text xml:lang="de">283</ojp:Text>
                      </ojp:PublishedJourneyNumber>
                      <ojp:OperatorName>
                        <ojp:Text xml:lang="de">Berner Oberland-Bahnen</ojp:Text>
                      </ojp:OperatorName>
                    </ojp:Extension>
                  </ojp:TimedLeg>
                </ojp:TripLeg>
              </ojp:Trip>
            </ojp:TripResult>
            <ojp:TripResult>
              <ojp:ResultId>ID-84C6B205-7933-4CC8-A59D-1D89314BE04E</ojp:ResultId>
              <ojp:Trip>
                <ojp:TripId>ID-84C6B205-7933-4CC8-A59D-1D89314BE04E</ojp:TripId>
                <ojp:Duration>PT2H2M</ojp:Duration>
                <ojp:StartTime>2022-12-12T16:07:00Z</ojp:StartTime>
                <ojp:EndTime>2022-12-12T18:09:00Z</ojp:EndTime>
                <ojp:Transfers>2</ojp:Transfers>
                <ojp:Distance>41561</ojp:Distance>
                <ojp:TripLeg>
                  <ojp:LegId>1</ojp:LegId>
                  <ojp:TimedLeg>
                    <ojp:LegBoard>
                      <siri:StopPointRef>8507000</siri:StopPointRef>
                      <ojp:StopPointName>
                        <ojp:Text xml:lang="de">Bern</ojp:Text>
                      </ojp:StopPointName>
                      <ojp:PlannedQuay>
                        <ojp:Text xml:lang="de">7</ojp:Text>
                      </ojp:PlannedQuay>
                      <ojp:ServiceDeparture>
                        <ojp:TimetabledTime>2022-12-12T16:07:00Z</ojp:TimetabledTime>
                      </ojp:ServiceDeparture>
                      <ojp:Order>1</ojp:Order>
                    </ojp:LegBoard>
                    <ojp:LegIntermediates>
                      <siri:StopPointRef>8507100</siri:StopPointRef>
                      <ojp:StopPointName>
                        <ojp:Text xml:lang="de">Thun</ojp:Text>
                      </ojp:StopPointName>
                      <ojp:PlannedQuay>
                        <ojp:Text xml:lang="de">2</ojp:Text>
                      </ojp:PlannedQuay>
                      <ojp:ServiceArrival>
                        <ojp:TimetabledTime>2022-12-12T16:25:00Z</ojp:TimetabledTime>
                      </ojp:ServiceArrival>
                      <ojp:ServiceDeparture>
                        <ojp:TimetabledTime>2022-12-12T16:26:00Z</ojp:TimetabledTime>
                      </ojp:ServiceDeparture>
                      <ojp:Order>2</ojp:Order>
                    </ojp:LegIntermediates>
                    <ojp:LegAlight>
                      <siri:StopPointRef>8507483</siri:StopPointRef>
                      <ojp:StopPointName>
                        <ojp:Text xml:lang="de">Spiez</ojp:Text>
                      </ojp:StopPointName>
                      <ojp:PlannedQuay>
                        <ojp:Text xml:lang="de">3</ojp:Text>
                      </ojp:PlannedQuay>
                      <ojp:ServiceArrival>
                        <ojp:TimetabledTime>2022-12-12T16:36:00Z</ojp:TimetabledTime>
                      </ojp:ServiceArrival>
                      <ojp:Order>3</ojp:Order>
                    </ojp:LegAlight>
                    <ojp:Service>
                      <ojp:OperatingDayRef>2022-12-12</ojp:OperatingDayRef>
                      <ojp:JourneyRef>ojp:91008:E:R:j23:349:824</ojp:JourneyRef>
                      <siri:LineRef>ojp:91008:E</siri:LineRef>
                      <siri:DirectionRef>R</siri:DirectionRef>
                      <ojp:Mode>
                        <ojp:PtMode>rail</ojp:PtMode>
                        <siri:RailSubmode>interRegionalRailService</siri:RailSubmode>
                        <ojp:Name>
                          <ojp:Text xml:lang="de">Zug</ojp:Text>
                        </ojp:Name>
                        <ojp:ShortName>
                          <ojp:Text xml:lang="de">IC</ojp:Text>
                        </ojp:ShortName>
                      </ojp:Mode>
                      <ojp:PublishedLineName>
                        <ojp:Text xml:lang="de">IC8</ojp:Text>
                      </ojp:PublishedLineName>
                      <ojp:OperatorRef>ojp:11</ojp:OperatorRef>
                      <ojp:Attribute>
                        <ojp:Text>
                          <ojp:Text xml:lang="de">Businesszone in 1. Klasse</ojp:Text>
                        </ojp:Text>
                        <ojp:Code>A__BZ</ojp:Code>
                        <siri:FareClassFacility>firstClass</siri:FareClassFacility>
                        <siri:PassengerCommsFacility>businessServices</siri:PassengerCommsFacility>
                      </ojp:Attribute>
                      <ojp:Attribute>
                        <ojp:Text>
                          <ojp:Text xml:lang="de">Gratis-Internet mit der App SBB FreeSurf</ojp:Text>
                        </ojp:Text>
                        <ojp:Code>A__FS</ojp:Code>
                      </ojp:Attribute>
                      <ojp:Attribute>
                        <ojp:Text>
                          <ojp:Text xml:lang="de">Ruhezone in 1. Klasse</ojp:Text>
                        </ojp:Text>
                        <ojp:Code>A__RZ</ojp:Code>
                        <siri:NuisanceFacility>mobilePhoneFreeZone</siri:NuisanceFacility>
                      </ojp:Attribute>
                      <ojp:Attribute>
                        <ojp:Text>
                          <ojp:Text xml:lang="de">Familienwagen mit Spielplatz</ojp:Text>
                        </ojp:Text>
                        <ojp:Code>A__FA</ojp:Code>
                      </ojp:Attribute>
                      <ojp:Attribute>
                        <ojp:Text>
                          <ojp:Text xml:lang="de">Platzreservierung möglich</ojp:Text>
                        </ojp:Text>
                        <ojp:Code>A___R</ojp:Code>
                      </ojp:Attribute>
                      <ojp:Attribute>
                        <ojp:Text>
                          <ojp:Text xml:lang="de">Restaurant</ojp:Text>
                        </ojp:Text>
                        <ojp:Code>A__WR</ojp:Code>
                        <siri:RefreshmentFacility>restaurantService</siri:RefreshmentFacility>
                      </ojp:Attribute>
                      <ojp:Attribute>
                        <ojp:Text>
                          <ojp:Text xml:lang="de">Aussteigeseite: Rechts</ojp:Text>
                        </ojp:Text>
                        <ojp:Code>ojp91008ER_InfoCall349_111055_1</ojp:Code>
                      </ojp:Attribute>
                      <ojp:Attribute>
                        <ojp:Text>
                          <ojp:Text xml:lang="de">Aussteigeseite: Links</ojp:Text>
                        </ojp:Text>
                        <ojp:Code>ojp91008ER_InfoCall349_111340_1</ojp:Code>
                      </ojp:Attribute>
                      <ojp:DestinationStopPointRef>8501609</ojp:DestinationStopPointRef>
                      <ojp:DestinationText>
                        <ojp:Text xml:lang="de">Brig</ojp:Text>
                      </ojp:DestinationText>
                    </ojp:Service>
                    <ojp:Extension>
                      <ojp:TransportTypeName>
                        <ojp:Text xml:lang="de">InterCity</ojp:Text>
                      </ojp:TransportTypeName>
                      <ojp:PublishedJourneyNumber>
                        <ojp:Text xml:lang="de">824</ojp:Text>
                      </ojp:PublishedJourneyNumber>
                      <ojp:OperatorName>
                        <ojp:Text xml:lang="de">Schweizerische Bundesbahnen SBB</ojp:Text>
                      </ojp:OperatorName>
                    </ojp:Extension>
                  </ojp:TimedLeg>
                </ojp:TripLeg>
                <ojp:TripLeg>
                  <ojp:LegId>2</ojp:LegId>
                  <ojp:TransferLeg>
                    <ojp:TransferMode>walk</ojp:TransferMode>
                    <ojp:LegStart>
                      <siri:StopPointRef>8507483</siri:StopPointRef>
                      <ojp:LocationName>
                        <ojp:Text xml:lang="de">Spiez</ojp:Text>
                      </ojp:LocationName>
                    </ojp:LegStart>
                    <ojp:LegEnd>
                      <siri:StopPointRef>8507483</siri:StopPointRef>
                      <ojp:LocationName>
                        <ojp:Text xml:lang="de">Spiez</ojp:Text>
                      </ojp:LocationName>
                    </ojp:LegEnd>
                    <ojp:TimeWindowStart>2022-12-12T16:36:00Z</ojp:TimeWindowStart>
                    <ojp:TimeWindowEnd>2022-12-12T17:05:00Z</ojp:TimeWindowEnd>
                    <ojp:Duration>PT29M</ojp:Duration>
                    <ojp:WalkDuration>PT4M</ojp:WalkDuration>
                    <ojp:BufferTime>PT25M</ojp:BufferTime>
                  </ojp:TransferLeg>
                </ojp:TripLeg>
                <ojp:TripLeg>
                  <ojp:LegId>3</ojp:LegId>
                  <ojp:TimedLeg>
                    <ojp:LegBoard>
                      <siri:StopPointRef>8507483</siri:StopPointRef>
                      <ojp:StopPointName>
                        <ojp:Text xml:lang="de">Spiez</ojp:Text>
                      </ojp:StopPointName>
                      <ojp:PlannedQuay>
                        <ojp:Text xml:lang="de">2</ojp:Text>
                      </ojp:PlannedQuay>
                      <ojp:ServiceDeparture>
                        <ojp:TimetabledTime>2022-12-12T17:05:00Z</ojp:TimetabledTime>
                      </ojp:ServiceDeparture>
                      <ojp:Order>1</ojp:Order>
                    </ojp:LegBoard>
                    <ojp:LegIntermediates>
                      <siri:StopPointRef>8507493</siri:StopPointRef>
                      <ojp:StopPointName>
                        <ojp:Text xml:lang="de">Interlaken West</ojp:Text>
                      </ojp:StopPointName>
                      <ojp:PlannedQuay>
                        <ojp:Text xml:lang="de">1</ojp:Text>
                      </ojp:PlannedQuay>
                      <ojp:ServiceArrival>
                        <ojp:TimetabledTime>2022-12-12T17:21:00Z</ojp:TimetabledTime>
                      </ojp:ServiceArrival>
                      <ojp:ServiceDeparture>
                        <ojp:TimetabledTime>2022-12-12T17:22:00Z</ojp:TimetabledTime>
                      </ojp:ServiceDeparture>
                      <ojp:Order>2</ojp:Order>
                    </ojp:LegIntermediates>
                    <ojp:LegAlight>
                      <siri:StopPointRef>8507492</siri:StopPointRef>
                      <ojp:StopPointName>
                        <ojp:Text xml:lang="de">Interlaken Ost</ojp:Text>
                      </ojp:StopPointName>
                      <ojp:PlannedQuay>
                        <ojp:Text xml:lang="de">8</ojp:Text>
                      </ojp:PlannedQuay>
                      <ojp:ServiceArrival>
                        <ojp:TimetabledTime>2022-12-12T17:26:00Z</ojp:TimetabledTime>
                      </ojp:ServiceArrival>
                      <ojp:Order>3</ojp:Order>
                    </ojp:LegAlight>
                    <ojp:Service>
                      <ojp:OperatingDayRef>2022-12-12</ojp:OperatingDayRef>
                      <ojp:JourneyRef>ojp:91022:Y:H:j23:166:4232</ojp:JourneyRef>
                      <siri:LineRef>ojp:91022:Y</siri:LineRef>
                      <siri:DirectionRef>H</siri:DirectionRef>
                      <ojp:Mode>
                        <ojp:PtMode>rail</ojp:PtMode>
                        <siri:RailSubmode>regionalRail</siri:RailSubmode>
                        <ojp:Name>
                          <ojp:Text xml:lang="de">Zug</ojp:Text>
                        </ojp:Name>
                        <ojp:ShortName>
                          <ojp:Text xml:lang="de">RE</ojp:Text>
                        </ojp:ShortName>
                      </ojp:Mode>
                      <ojp:PublishedLineName>
                        <ojp:Text xml:lang="de">RE</ojp:Text>
                      </ojp:PublishedLineName>
                      <ojp:OperatorRef>ojp:33</ojp:OperatorRef>
                      <ojp:Attribute>
                        <ojp:Text>
                          <ojp:Text xml:lang="de">Aussteigeseite: Rechts</ojp:Text>
                        </ojp:Text>
                        <ojp:Code>ojp91022YH_InfoCall166_111343_1</ojp:Code>
                      </ojp:Attribute>
                      <ojp:DestinationStopPointRef>8507492</ojp:DestinationStopPointRef>
                      <ojp:DestinationText>
                        <ojp:Text xml:lang="de">Interlaken Ost</ojp:Text>
                      </ojp:DestinationText>
                    </ojp:Service>
                    <ojp:Extension>
                      <ojp:TransportTypeName>
                        <ojp:Text xml:lang="de">RegioExpress</ojp:Text>
                      </ojp:TransportTypeName>
                      <ojp:PublishedJourneyNumber>
                        <ojp:Text xml:lang="de">4232</ojp:Text>
                      </ojp:PublishedJourneyNumber>
                      <ojp:OperatorName>
                        <ojp:Text xml:lang="de">BLS AG (bls)</ojp:Text>
                      </ojp:OperatorName>
                    </ojp:Extension>
                  </ojp:TimedLeg>
                </ojp:TripLeg>
                <ojp:TripLeg>
                  <ojp:LegId>4</ojp:LegId>
                  <ojp:TransferLeg>
                    <ojp:TransferMode>walk</ojp:TransferMode>
                    <ojp:LegStart>
                      <siri:StopPointRef>8507492</siri:StopPointRef>
                      <ojp:LocationName>
                        <ojp:Text xml:lang="de">Interlaken Ost</ojp:Text>
                      </ojp:LocationName>
                    </ojp:LegStart>
                    <ojp:LegEnd>
                      <siri:StopPointRef>8507492</siri:StopPointRef>
                      <ojp:LocationName>
                        <ojp:Text xml:lang="de">Interlaken Ost</ojp:Text>
                      </ojp:LocationName>
                    </ojp:LegEnd>
                    <ojp:TimeWindowStart>2022-12-12T17:26:00Z</ojp:TimeWindowStart>
                    <ojp:TimeWindowEnd>2022-12-12T17:35:00Z</ojp:TimeWindowEnd>
                    <ojp:Duration>PT9M</ojp:Duration>
                    <ojp:WalkDuration>PT5M</ojp:WalkDuration>
                    <ojp:BufferTime>PT4M</ojp:BufferTime>
                  </ojp:TransferLeg>
                </ojp:TripLeg>
                <ojp:TripLeg>
                  <ojp:LegId>5</ojp:LegId>
                  <ojp:TimedLeg>
                    <ojp:LegBoard>
                      <siri:StopPointRef>8507492</siri:StopPointRef>
                      <ojp:StopPointName>
                        <ojp:Text xml:lang="de">Interlaken Ost</ojp:Text>
                      </ojp:StopPointName>
                      <ojp:PlannedQuay>
                        <ojp:Text xml:lang="de">2B</ojp:Text>
                      </ojp:PlannedQuay>
                      <ojp:ServiceDeparture>
                        <ojp:TimetabledTime>2022-12-12T17:35:00Z</ojp:TimetabledTime>
                      </ojp:ServiceDeparture>
                      <ojp:Order>1</ojp:Order>
                    </ojp:LegBoard>
                    <ojp:LegIntermediates>
                      <siri:StopPointRef>8507388</siri:StopPointRef>
                      <ojp:StopPointName>
                        <ojp:Text xml:lang="de">Wilderswil</ojp:Text>
                      </ojp:StopPointName>
                      <ojp:PlannedQuay>
                        <ojp:Text xml:lang="de">2CD</ojp:Text>
                      </ojp:PlannedQuay>
                      <ojp:ServiceArrival>
                        <ojp:TimetabledTime>2022-12-12T17:39:00Z</ojp:TimetabledTime>
                      </ojp:ServiceArrival>
                      <ojp:ServiceDeparture>
                        <ojp:TimetabledTime>2022-12-12T17:40:00Z</ojp:TimetabledTime>
                      </ojp:ServiceDeparture>
                      <ojp:Order>2</ojp:Order>
                    </ojp:LegIntermediates>
                    <ojp:LegIntermediates>
                      <siri:StopPointRef>8507389</siri:StopPointRef>
                      <ojp:StopPointName>
                        <ojp:Text xml:lang="de">Zweilütschinen</ojp:Text>
                      </ojp:StopPointName>
                      <ojp:PlannedQuay>
                        <ojp:Text xml:lang="de">3CD</ojp:Text>
                      </ojp:PlannedQuay>
                      <ojp:ServiceArrival>
                        <ojp:TimetabledTime>2022-12-12T17:45:00Z</ojp:TimetabledTime>
                      </ojp:ServiceArrival>
                      <ojp:ServiceDeparture>
                        <ojp:TimetabledTime>2022-12-12T17:47:00Z</ojp:TimetabledTime>
                      </ojp:ServiceDeparture>
                      <ojp:Order>3</ojp:Order>
                    </ojp:LegIntermediates>
                    <ojp:LegIntermediates>
                      <siri:StopPointRef>8507383</siri:StopPointRef>
                      <ojp:StopPointName>
                        <ojp:Text xml:lang="de">Lütschental</ojp:Text>
                      </ojp:StopPointName>
                      <ojp:PlannedQuay>
                        <ojp:Text xml:lang="de">2</ojp:Text>
                      </ojp:PlannedQuay>
                      <ojp:ServiceArrival>
                        <ojp:TimetabledTime>2022-12-12T17:52:00Z</ojp:TimetabledTime>
                      </ojp:ServiceArrival>
                      <ojp:ServiceDeparture>
                        <ojp:TimetabledTime>2022-12-12T17:52:00Z</ojp:TimetabledTime>
                      </ojp:ServiceDeparture>
                      <ojp:Order>4</ojp:Order>
                    </ojp:LegIntermediates>
                    <ojp:LegIntermediates>
                      <siri:StopPointRef>8507382</siri:StopPointRef>
                      <ojp:StopPointName>
                        <ojp:Text xml:lang="de">Burglauenen</ojp:Text>
                      </ojp:StopPointName>
                      <ojp:PlannedQuay>
                        <ojp:Text xml:lang="de">2</ojp:Text>
                      </ojp:PlannedQuay>
                      <ojp:ServiceArrival>
                        <ojp:TimetabledTime>2022-12-12T17:57:00Z</ojp:TimetabledTime>
                      </ojp:ServiceArrival>
                      <ojp:ServiceDeparture>
                        <ojp:TimetabledTime>2022-12-12T17:58:00Z</ojp:TimetabledTime>
                      </ojp:ServiceDeparture>
                      <ojp:Order>5</ojp:Order>
                    </ojp:LegIntermediates>
                    <ojp:LegIntermediates>
                      <siri:StopPointRef>8507381</siri:StopPointRef>
                      <ojp:StopPointName>
                        <ojp:Text xml:lang="de">Schwendi bei Grindelwald</ojp:Text>
                      </ojp:StopPointName>
                      <ojp:PlannedQuay>
                        <ojp:Text xml:lang="de">2</ojp:Text>
                      </ojp:PlannedQuay>
                      <ojp:ServiceArrival>
                        <ojp:TimetabledTime>2022-12-12T18:01:00Z</ojp:TimetabledTime>
                      </ojp:ServiceArrival>
                      <ojp:ServiceDeparture>
                        <ojp:TimetabledTime>2022-12-12T18:01:00Z</ojp:TimetabledTime>
                      </ojp:ServiceDeparture>
                      <ojp:Order>6</ojp:Order>
                    </ojp:LegIntermediates>
                    <ojp:LegIntermediates>
                      <siri:StopPointRef>8505226</siri:StopPointRef>
                      <ojp:StopPointName>
                        <ojp:Text xml:lang="de">Grindelwald Terminal</ojp:Text>
                      </ojp:StopPointName>
                      <ojp:ServiceArrival>
                        <ojp:TimetabledTime>2022-12-12T18:04:00Z</ojp:TimetabledTime>
                      </ojp:ServiceArrival>
                      <ojp:ServiceDeparture>
                        <ojp:TimetabledTime>2022-12-12T18:05:00Z</ojp:TimetabledTime>
                      </ojp:ServiceDeparture>
                      <ojp:Order>7</ojp:Order>
                    </ojp:LegIntermediates>
                    <ojp:LegAlight>
                      <siri:StopPointRef>8507380</siri:StopPointRef>
                      <ojp:StopPointName>
                        <ojp:Text xml:lang="de">Grindelwald</ojp:Text>
                      </ojp:StopPointName>
                      <ojp:PlannedQuay>
                        <ojp:Text xml:lang="de">1</ojp:Text>
                      </ojp:PlannedQuay>
                      <ojp:ServiceArrival>
                        <ojp:TimetabledTime>2022-12-12T18:09:00Z</ojp:TimetabledTime>
                      </ojp:ServiceArrival>
                      <ojp:Order>8</ojp:Order>
                    </ojp:LegAlight>
                    <ojp:Service>
                      <ojp:OperatingDayRef>2022-12-12</ojp:OperatingDayRef>
                      <ojp:JourneyRef>ojp:91061::H:j23:129:285</ojp:JourneyRef>
                      <siri:LineRef>ojp:91061:</siri:LineRef>
                      <siri:DirectionRef>H</siri:DirectionRef>
                      <ojp:Mode>
                        <ojp:PtMode>rail</ojp:PtMode>
                        <siri:RailSubmode>regionalRail</siri:RailSubmode>
                        <ojp:Name>
                          <ojp:Text xml:lang="de">Zug</ojp:Text>
                        </ojp:Name>
                        <ojp:ShortName>
                          <ojp:Text xml:lang="de">R</ojp:Text>
                        </ojp:ShortName>
                      </ojp:Mode>
                      <ojp:PublishedLineName>
                        <ojp:Text xml:lang="de">R61</ojp:Text>
                      </ojp:PublishedLineName>
                      <ojp:OperatorRef>ojp:35</ojp:OperatorRef>
                      <ojp:DestinationStopPointRef>8507380</ojp:DestinationStopPointRef>
                      <ojp:DestinationText>
                        <ojp:Text xml:lang="de">Grindelwald</ojp:Text>
                      </ojp:DestinationText>
                    </ojp:Service>
                    <ojp:Extension>
                      <ojp:TransportTypeName>
                        <ojp:Text xml:lang="de">Regio</ojp:Text>
                      </ojp:TransportTypeName>
                      <ojp:PublishedJourneyNumber>
                        <ojp:Text xml:lang="de">285</ojp:Text>
                      </ojp:PublishedJourneyNumber>
                      <ojp:OperatorName>
                        <ojp:Text xml:lang="de">Berner Oberland-Bahnen</ojp:Text>
                      </ojp:OperatorName>
                    </ojp:Extension>
                  </ojp:TimedLeg>
                </ojp:TripLeg>
              </ojp:Trip>
            </ojp:TripResult>
          </ojp:OJPTripDelivery>
        </siri:ServiceDelivery>
      </siri:OJPResponse>
    </siri:OJP>"#,
    );
    let trips = trip_result.unwrap().get_trips();
    assert_eq!("hello", "hello");
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
        System::AT.shared_adjacency(System::IT).sort(),
        vec![System::CH, System::SLO].sort()
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
