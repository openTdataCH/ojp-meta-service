use chrono::{DateTime, SecondsFormat, Utc};
use reqwest::Client;
use std::{str::FromStr, time::SystemTime};

use crate::types::{ErrorResponse, OjpDoc, Point, System, SystemConfig, Trip};

pub fn format_lir(
    query: &str,
    requestor_ref: &str,
    nr_of_results: usize,
    include_pt_modes: bool,
) -> String {
    let timestamp =
        DateTime::<Utc>::from(SystemTime::now()).to_rfc3339_opts(SecondsFormat::Millis, true);
    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
  <OJP xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema" xmlns="http://www.siri.org.uk/siri" version="1.0" xmlns:ojp="http://www.vdv.de/ojp" xsi:schemaLocation="http://www.siri.org.uk/siri ../ojp-xsd-v1.0/OJP.xsd">
      <OJPRequest>
          <ServiceRequest>
              <RequestTimestamp>{timestamp}</RequestTimestamp>
              <RequestorRef>{requestor_ref}</RequestorRef>
              <ojp:OJPLocationInformationRequest>
                  <RequestTimestamp>{timestamp}</RequestTimestamp>
                  <ojp:InitialInput>
                      <ojp:LocationName>{query}</ojp:LocationName>
                  </ojp:InitialInput>
                  <ojp:Restrictions>
                      <ojp:Type>stop</ojp:Type>
                      <ojp:NumberOfResults>{nr_of_results}</ojp:NumberOfResults>
                      <ojp:IncludePtModes>{include_pt_modes}</ojp:IncludePtModes>
                  </ojp:Restrictions>
              </ojp:OJPLocationInformationRequest>
          </ServiceRequest>
      </OJPRequest>
  </OJP>"#
    )
}

//Construct for the Location Information Request. The following Parameters are dynamic:
//timestamp -> Typical timestamp telling the time of the request
//requestor_ref -> Reference, can be anything. Tells where the request is coming from. Some Endpoints use that for authentication.
pub fn format_epr(requestor_ref: &str) -> String {
    let timestamp =
        DateTime::<Utc>::from(SystemTime::now()).to_rfc3339_opts(SecondsFormat::Millis, true);
    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
    <OJP xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema" xmlns="http://www.siri.org.uk/siri" version="1.0" xmlns:ojp="http://www.vdv.de/ojp" xsi:schemaLocation="http://www.siri.org.uk/siri ../ojp-xsd-v1.0/OJP.xsd">
        <OJPRequest>
            <ServiceRequest>
                <RequestTimestamp>{timestamp}</RequestTimestamp>
                <RequestorRef>{requestor_ref}</RequestorRef>
                <ojp:OJPExchangePointsRequest>
                    <RequestTimestamp>{timestamp}</RequestTimestamp>
                       <ojp:Params>
                            <ojp:NumberOfResults>10000</ojp:NumberOfResults>
                            <ojp:ContinueAt>0</ojp:ContinueAt>
                       </ojp:Params>
                </ojp:OJPExchangePointsRequest>
            </ServiceRequest>
        </OJPRequest>
    </OJP>"#
    )
}

pub fn format_trip(
    origin_ref: &str,
    origin_name: &str,
    destination_ref: &str,
    destination_name: &str,
    requestor_ref: &str,
) -> String {
    let timestamp =
        DateTime::<Utc>::from(SystemTime::now()).to_rfc3339_opts(SecondsFormat::Millis, true);
    format!(
        r#"<?xml version="1.0" encoding="utf-8"?>
        <OJP xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema" xmlns="http://www.siri.org.uk/siri" version="1.0" xmlns:ojp="http://www.vdv.de/ojp" xsi:schemaLocation="http://www.siri.org.uk/siri ../ojp-xsd-v1.0/OJP.xsd">
          <OJPRequest>
            <ServiceRequest>
              <RequestTimestamp>{timestamp}</RequestTimestamp>
              <RequestorRef>{requestor_ref}</RequestorRef>
              <ojp:OJPTripRequest>
                <RequestTimestamp>{timestamp}</RequestTimestamp>
                <ojp:Origin>
                  <ojp:PlaceRef>
                    <ojp:StopPlaceRef>{origin_ref}</ojp:StopPlaceRef>
                    <ojp:LocationName>
                      <ojp:Text>{origin_name}</ojp:Text>
                    </ojp:LocationName>
                  </ojp:PlaceRef>
                  <ojp:DepArrTime>{timestamp}</ojp:DepArrTime>
                </ojp:Origin>
                <ojp:Destination>
                  <ojp:PlaceRef>
                    <ojp:StopPlaceRef>{destination_ref}</ojp:StopPlaceRef>
                    <ojp:LocationName>
                      <ojp:Text>{destination_name}</ojp:Text>
                    </ojp:LocationName>
                  </ojp:PlaceRef>
                </ojp:Destination>
                <ojp:Params>
                  <ojp:IncludeTrackSections></ojp:IncludeTrackSections>
                  <ojp:IncludeTurnDescription></ojp:IncludeTurnDescription>
                  <ojp:IncludeIntermediateStops>true</ojp:IncludeIntermediateStops>
                </ojp:Params>
              </ojp:OJPTripRequest>
            </ServiceRequest>
          </OJPRequest>
        </OJP>"#
    )
}

// helper function to get a trip
pub async fn get_trip(
    origin_ref: &str,
    origin_name: &str,
    destination_ref: &str,
    destination_name: &str,
    system: SystemConfig,
) -> Result<Vec<Trip>, ErrorResponse> {
    let res = Client::new()
        .post(system.url)
        .bearer_auth(system.key)
        .header("Content-Type", "text/xml")
        .body(format_trip(
            origin_ref,
            origin_name,
            destination_ref,
            destination_name,
            system.req_ref,
        ))
        .send()
        .await
        .or(Err("OJP-Service can't be reached...".to_string()))?
        .text()
        .await
        .or(Err("OJP response not readable...".to_string()))?;
    let doc = OjpDoc::new(&res)?;
    let trips = doc.get_trips()?;
    Ok(trips)
}
