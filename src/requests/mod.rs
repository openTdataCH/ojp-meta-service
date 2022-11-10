use chrono::{DateTime, SecondsFormat, Utc};
use std::time::SystemTime;

pub fn format_lir(query: &str, nr_of_results: usize, include_pt_modes: bool) -> String {
    let timestamp =
        DateTime::<Utc>::from(SystemTime::now()).to_rfc3339_opts(SecondsFormat::Millis, true);
    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
  <OJP xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema" xmlns="http://www.siri.org.uk/siri" version="1.0" xmlns:ojp="http://www.vdv.de/ojp" xsi:schemaLocation="http://www.siri.org.uk/siri ../ojp-xsd-v1.0/OJP.xsd">
      <OJPRequest>
          <ServiceRequest>
              <RequestTimestamp>{timestamp}</RequestTimestamp>
              <RequestorRef>OJP-META</RequestorRef>
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
                            <ojp:NumberOfResults>100</ojp:NumberOfResults>
                            <ojp:ContinueAt>0</ojp:ContinueAt>
                       </ojp:Params>
                </ojp:OJPExchangePointsRequest>
            </ServiceRequest>
        </OJPRequest>
    </OJP>"#
    )
}
