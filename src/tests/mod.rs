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

fn test_lir_request_ok() -> Result<(), ErrorResponse>{
    let system = System::CH.get_config();
    async move {
        let res = Client::new()
            .post(system.url)
            .bearer_auth(system.key)
            .header("Content-Type", "text/xml")
            .body(format_lir("Bern", 1, false))
            .send()
            .await
            .map_err(|_| ErrorResponse::ReqwestError("OJP-Service can't be reached...".to_string()))?
            .text()
            .await
            // with map_err we can map a reqwest error (which we can't control) to a custom error
            .map_err(|_| {
                ErrorResponse::ReqwestError("OJP-Service repsonse can't be read...".to_string())
            })?;
    };

    

    let doc = roxmltree::Document::parse(&res).unwrap();
    let nodes = doc
        .descendants()
        .find(|n| n.has_tag_name("OJPLocationInformationDelivery"))
        .and_then(|f| {
            Some(
                f.children()
                    .filter(|n| n.has_tag_name("Location"))
                    .collect::<Vec<roxmltree::Node>>(),
            )
        })
        .unwrap();
    let locs = nodes
        .iter()
        .map(|n| parse_lir(&n))
        .collect::<Result<Vec<Location>, ErrorResponse>>()?;
    assert_eq!("1","1");
}

fn test_epr_request_ok() {

}