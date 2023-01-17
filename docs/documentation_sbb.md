# API Keys

In this project, we have to collect data from different (already existing) endpoints. Every endpoint requires a different API Key. The different API keys and their regarding endpoints are listed in this document.

| Endpoint     |          Path |Token|   Description|
|--------------|--------------|--------------|--------------|
|https://api.opentransportdata.swiss|/ojp2020|57c5dbbbf1fe4d0001000018e0f7158cb2b347e3a6745e3ef949e7bf|SBB Prod|
|https://odpch-test.cloud.tyk.io|/ojp-test|57c5dadd5e6307000100005ead6b87d5ec4f48d3ad5f9414e92907d4|SBB Test|
|https://dev.atob.si|/OpenAPI/LinkingAlps/OJP|Not needed|LUR (Slovenia)|
|https://efa.sta.bz.it  |/ojp/ojp|Not needed|STA LIR (Italy/Südtirol)|
|https://vao.demo.hafas.de|/linking-alps/ojp|Kein Token gebraucht, aber im <RequestorRef>-XML Field muss "LinkingAlps-Test-2022" stehen.|VAO LIR (Austria)|
|https://la-fvk.prod.ojp.odpch.ch|/ojp/ojp|57c5dbbbf1fe4d0001000018310ba4201a8042eb9cfba42c0a9cf01b|SBB long distance system, should only be used between exchange points with OJPMultiPointTripRequest|
|https://euspirit.demo.hafas.de|/rcc|Not needed|EU Spirit|

# Overview of OJP Requests 

The OJP Requests are the basis of the program. The Requests are needed to get all the data required. This data is later being analyzed and standartized up by the program.

All the OJP Requests are in XML Format. In this document, only the required requests for the program are being described. There are of course more requests but they are irrelevant for this project.

The Endpoints on which the requests can be performed are described above.

## Location Information Request (LIR)

### Request

These Requests are useful to obtain data about different Locations. A Location can be a bus stop, train stop and various other things. Here are some important parameters which might be useful for the project:

| Parameter | Description |
|--------------|--------------|
| RequestorRef | Tells the endpoint who requested. Some Endpoints even use this for Authentication. |
| LocationName | The Location to check. Can result in multiple Locations as response if it's not exact. If we put in "Bern" for example, every stop containing Bern in the Name will be displayed. |
| NumberOfResults | Maximum number of results to show |

```xml
<?xml version="1.0" encoding="UTF-8"?>
<OJP xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema" xmlns="http://www.siri.org.uk/siri" version="1.0" xmlns:ojp="http://www.vdv.de/ojp" xsi:schemaLocation="http://www.siri.org.uk/siri ../ojp-xsd-v1.0/OJP.xsd">
	<OJPRequest>
		<ServiceRequest>
			<RequestTimestamp>2022-01-31T13:45:09.541Z</RequestTimestamp>
			<RequestorRef>API-Explorer</RequestorRef>
			<ojp:OJPLocationInformationRequest>
				<RequestTimestamp>2022-01-31T13:45:09.541Z</RequestTimestamp>
				<ojp:InitialInput>
					<ojp:LocationName>Bern, Länggasse</ojp:LocationName>
				</ojp:InitialInput>
				<ojp:Restrictions>
					<ojp:Type>stop</ojp:Type>
					<ojp:NumberOfResults>10</ojp:NumberOfResults>
					<ojp:IncludePtModes>false</ojp:IncludePtModes>
				</ojp:Restrictions>
			</ojp:OJPLocationInformationRequest>
		</ServiceRequest>
	</OJPRequest>
</OJP>
```

### Response

After sending a request, we of course get a fitting response. Here's an example response of the request above. A description of the most important response parameters is also being shown in the table.

| Parameter | Description |
|--------------|--------------|
| StopPlaceRef | Probably the most important one. This is a reference number which is later being used for Trip Requests. |
| StopPlaceName | The exact name of the StopPlace.|
| LocationName | The Exact name of the Location. |
| Longitude | Longitude of the Location |
| Latitude | Latitude of the Location |

```xml
<?xml version="1.0" encoding="UTF-8"?>
<siri:OJP xmlns:siri="http://www.siri.org.uk/siri" xmlns:ojp="http://www.vdv.de/ojp" version="1.0">
    <siri:OJPResponse>
        <siri:ServiceDelivery>
            <siri:ResponseTimestamp>2022-10-25T15:18:45Z</siri:ResponseTimestamp>
            <siri:ProducerRef>EFAController10.5.16.8-OJP-EFA01-P</siri:ProducerRef>
            <siri:Status>true</siri:Status>
            <ojp:OJPLocationInformationDelivery>
                <siri:ResponseTimestamp>2022-10-25T15:18:45Z</siri:ResponseTimestamp>
                <siri:Status>true</siri:Status>
                <ojp:CalcTime>77</ojp:CalcTime>
                <ojp:Location>
                    <ojp:Location>
                        <ojp:StopPlace>
                            <ojp:StopPlaceRef>8571359</ojp:StopPlaceRef>
                            <ojp:StopPlaceName>
                                <ojp:Text xml:lang="de">Bern, Länggasse</ojp:Text>
                            </ojp:StopPlaceName>
                            <ojp:TopographicPlaceRef>23006351:1</ojp:TopographicPlaceRef>
                        </ojp:StopPlace>
                        <ojp:LocationName>
                            <ojp:Text xml:lang="de">Bern, Länggasse (Bern)</ojp:Text>
                        </ojp:LocationName>
                        <ojp:GeoPosition>
                            <siri:Longitude>7.42617</siri:Longitude>
                            <siri:Latitude>46.95597</siri:Latitude>
                        </ojp:GeoPosition>
                    </ojp:Location>
                    <ojp:Complete>true</ojp:Complete>
                    <ojp:Probability>1</ojp:Probability>
                </ojp:Location>
            </ojp:OJPLocationInformationDelivery>
        </siri:ServiceDelivery>
    </siri:OJPResponse>
</siri:OJP>
```

## Trip Request (TR)

Trip Requests can be used to plan a trip from place A to place B. The response may consist of multiple options. In the examples below, a trip from Bern (Main Station) to Biel (Main Station) is being planned.

### Request

The request basically consists of a start and an ending point. Important parameters and an example are shown below.

| Parameter | Description |
|--------------|--------------|
| StopPlaceRef | The most important Parameter. Tells the Endpoint from which places the trip should start and end. |
| DepArrTime | Tells the endpoint when the train/bus/... should arrive at the ending point. |
| IncludeIntermediateStops | Can be set to true or false. Shows intermediate stops of the trip (or not). |
... The Location names aren't too important. 

```xml
<?xml version="1.0" encoding="utf-8"?>
<OJP xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema" xmlns="http://www.siri.org.uk/siri" version="1.0" xmlns:ojp="http://www.vdv.de/ojp" xsi:schemaLocation="http://www.siri.org.uk/siri ../ojp-xsd-v1.0/OJP.xsd">
  <OJPRequest>
    <ServiceRequest>
      <RequestTimestamp>2022-10-24T14:48:26.530Z</RequestTimestamp>
      <RequestorRef>API-Explorer</RequestorRef>
      <ojp:OJPTripRequest>
        <RequestTimestamp>2022-10-24T14:48:26.530Z</RequestTimestamp>
        <ojp:Origin>
          <ojp:PlaceRef>
            <ojp:StopPlaceRef>8507000</ojp:StopPlaceRef>
            <ojp:LocationName>
              <ojp:Text>Bern</ojp:Text>
            </ojp:LocationName>
          </ojp:PlaceRef>
          <ojp:DepArrTime>2022-10-26T16:48:09</ojp:DepArrTime>
        </ojp:Origin>
        <ojp:Destination>
          <ojp:PlaceRef>
            <ojp:StopPlaceRef>8504300</ojp:StopPlaceRef>
            <ojp:LocationName>
              <ojp:Text>Biel</ojp:Text>
            </ojp:LocationName>
          </ojp:PlaceRef>
        </ojp:Destination>
        <ojp:Params>
          <ojp:IncludeTrackSections></ojp:IncludeTrackSections>
          <ojp:IncludeTurnDescription></ojp:IncludeTurnDescription>
          <ojp:IncludeIntermediateStops></ojp:IncludeIntermediateStops>
        </ojp:Params>
      </ojp:OJPTripRequest>
    </ServiceRequest>
  </OJPRequest>
</OJP>
```

### Response

The whole response is too big to put it in this documentation. A lot of parts from the response are irrelevant for this project. Only the important parts are being highlighted and described. The important part are multiple TripResults which show us actual train connections from Bern to Biel.

| Parameter | Description |
|--------------|--------------|
| Duration | Duration of the trip. Normally in a format like "PT29M" = 29 Minutes, "PT3H" = 3 Hours, and so on.|
| StartTime | Department Time of the trip. |
| EndTime | Arrival Time of the trip. |
| Transfers | How many transfers are needed for the trip. |
| PlannedQuay | From which quay does the train depart/at which quay does it arrive. |

```xml
				<ojp:TripResult>
					<ojp:ResultId>ID-79CEC3EF-0698-4841-9251-641D9C24871D</ojp:ResultId>
					<ojp:Trip>
						<ojp:TripId>ID-79CEC3EF-0698-4841-9251-641D9C24871D</ojp:TripId>
						<ojp:Duration>PT29M</ojp:Duration>
						<ojp:StartTime>2022-10-26T14:46:00Z</ojp:StartTime>
						<ojp:EndTime>2022-10-26T15:15:00Z</ojp:EndTime>
						<ojp:Transfers>0</ojp:Transfers>
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
										<ojp:Text xml:lang="de">13AB</ojp:Text>
									</ojp:PlannedQuay>
									<ojp:ServiceDeparture>
										<ojp:TimetabledTime>2022-10-26T14:46:00Z</ojp:TimetabledTime>
									</ojp:ServiceDeparture>
									<ojp:Order>1</ojp:Order>
								</ojp:LegBoard>
								<ojp:LegAlight>
									<siri:StopPointRef>8504300</siri:StopPointRef>
									<ojp:StopPointName>
										<ojp:Text xml:lang="de">Biel/Bienne</ojp:Text>
									</ojp:StopPointName>
									<ojp:PlannedQuay>
										<ojp:Text xml:lang="de">9</ojp:Text>
									</ojp:PlannedQuay>
									<ojp:ServiceArrival>
										<ojp:TimetabledTime>2022-10-26T15:15:00Z</ojp:TimetabledTime>
									</ojp:ServiceArrival>
									<ojp:Order>7</ojp:Order>
								</ojp:LegAlight>
								<ojp:Service>
									<ojp:OperatingDayRef>2022-10-26</ojp:OperatingDayRef>
									<ojp:JourneyRef>ojp:91031::R:j22:188:16363</ojp:JourneyRef>
									<siri:LineRef>ojp:91031:</siri:LineRef>
									<siri:DirectionRef>R</siri:DirectionRef>
									<ojp:Mode>
										<ojp:PtMode>rail</ojp:PtMode>
										<siri:RailSubmode>regionalRail</siri:RailSubmode>
										<ojp:Name>
											<ojp:Text xml:lang="de">Zug</ojp:Text>
										</ojp:Name>
										<ojp:ShortName>
											<ojp:Text xml:lang="de">S</ojp:Text>
										</ojp:ShortName>
									</ojp:Mode>
									<ojp:PublishedLineName>
										<ojp:Text xml:lang="de">S31</ojp:Text>
									</ojp:PublishedLineName>
									<ojp:OperatorRef>ojp:33</ojp:OperatorRef>
									<ojp:DestinationStopPointRef>8504300</ojp:DestinationStopPointRef>
									<ojp:DestinationText>
										<ojp:Text xml:lang="de">Biel/Bienne</ojp:Text>
									</ojp:DestinationText>
								</ojp:Service>
								<ojp:Extension>
									<ojp:TransportTypeName>
										<ojp:Text xml:lang="de">S-Bahn</ojp:Text>
									</ojp:TransportTypeName>
									<ojp:PublishedJourneyNumber>
										<ojp:Text xml:lang="de">16363</ojp:Text>
									</ojp:PublishedJourneyNumber>
									<ojp:OperatorName>
										<ojp:Text xml:lang="de">BLS AG (bls)</ojp:Text>
									</ojp:OperatorName>
								</ojp:Extension>
							</ojp:TimedLeg>
						</ojp:TripLeg>
					</ojp:Trip>
				</ojp:TripResult>
```

The response usually consists of multiple TripResults. Only one is shown in the example above for simplicity.

## Multi Trip Request (MTR)

Multi Trip Requests are made if the user wants to add a "via" to his/her trip. 

### Request

The request is quite similar to the normal Trip request. The only difference is that there's one more StopPlaceRef added to the request. The more detailed information isn't necessary to be described again - it can be looked op on the TR-Chapter.

One important thing to mention would be that the VAO-System only understands the MultiPointType "anyPoint".

```xml
<?xml version="1.0" encoding="UTF-8"?>
<OJP
	xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
	xmlns:xsd="http://www.w3.org/2001/XMLSchema"
	xmlns="http://www.siri.org.uk/siri" version="1.0"
	xmlns:ojp="http://www.vdv.de/ojp" xsi:schemaLocation="http://www.siri.org.uk/siri ../ojp-xsd-v1.0/OJP.xsd">
	<siri:OJPRequest>
		<siri:ServiceRequest>
			<siri:RequestTimestamp>2022-10-24T14:48:26.530Z</siri:RequestTimestamp>
			<siri:RequestorRef>EFAController10.5.0.0-STR-DB-20</siri:RequestorRef>
			<siri:MessageIdentifier>D7E409B7-3819-4061-BFB6-F03E864F84AE</siri:MessageIdentifier>
			<ojp:OJPMultiPointTripRequest>
				<siri:RequestTimestamp>2022-10-24T14:48:26.530Z</siri:RequestTimestamp>
				<ojp:Origin>
					<ojp:PlaceRef>
						<ojp:StopPlaceRef>8503091</ojp:StopPlaceRef>
						<ojp:LocationName>
							<ojp:Text xml:lang="de">Zürich Giesshübel</ojp:Text>
						</ojp:LocationName>
					</ojp:PlaceRef>
					<ojp:DepArrTime>2022-10-26T19:48:26.530Z</ojp:DepArrTime>
				</ojp:Origin>
				<ojp:Destination>
					<ojp:PlaceRef>
						<ojp:StopPlaceRef>it:22021:36</ojp:StopPlaceRef>
						<ojp:LocationName>
							<ojp:Text xml:lang="de">Mals, Bahnhof/Malles</ojp:Text>
						</ojp:LocationName>
					</ojp:PlaceRef>
				</ojp:Destination>
				<ojp:Destination>
					<ojp:PlaceRef>
						<ojp:StopPlaceRef>8101236</ojp:StopPlaceRef>
						<ojp:LocationName>
							<ojp:Text xml:lang="de">Feldkirch</ojp:Text>
						</ojp:LocationName>
					</ojp:PlaceRef>
				</ojp:Destination>
				<ojp:Params>
					<ojp:NumberOfResults>1</ojp:NumberOfResults>
					<ojp:OptimisationMethod>fastest</ojp:OptimisationMethod>
					<ojp:MultiPointType>eachDestination</ojp:MultiPointType>
					<ojp:IncludeIntermediateStops>true</ojp:IncludeIntermediateStops>
				</ojp:Params>
			</ojp:OJPMultiPointTripRequest>
		</siri:ServiceRequest>
	</siri:OJPRequest>
</OJP>
```

### Response

The Response is similar to the one from the normal TR except there are usually more results. 

## Exchange Point Request (EPR)

The response of an EPR gives us a list of available exchange points in the system. The official definition of an exchange is that it's a point of which several systems know of.

### Request

The request is quite simple. the only thing to adjust if the NumberOfResults-Parameter which is quite self-describing. It's just the maximum number of results to display in the response.

```xml
<?xml version="1.0" encoding="UTF-8"?>
<OJP xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema" xmlns="http://www.siri.org.uk/siri" version="1.0" xmlns:ojp="http://www.vdv.de/ojp" xsi:schemaLocation="http://www.siri.org.uk/siri ../ojp-xsd-v1.0/OJP.xsd">
    <OJPRequest>
        <ServiceRequest>
            <RequestTimestamp>2022-01-31T13:45:09.541Z</RequestTimestamp>
            <RequestorRef>LinkingAlps-Test-2022</RequestorRef>
            <ojp:OJPExchangePointsRequest>
                <RequestTimestamp>2022-01-31T13:45:09.541Z</RequestTimestamp>
                   <ojp:Params>
                        <ojp:NumberOfResults>100</ojp:NumberOfResults>
                        <ojp:ContinueAt>0</ojp:ContinueAt>
                   </ojp:Params>
            </ojp:OJPExchangePointsRequest>
        </ServiceRequest>
    </OJPRequest>
</OJP>
```


### Response

The response gives us a list of exchange points from the system. The response and it's most important parameters are described below.

| Parameter | Description |
|--------------|--------------|
| PrivateCode -> Value | An "international" reference number. |
| Longitude | Longitude of the place. |
| Latitude | Latitude of the place. |
| LocationName -> Text | The name of the place. |
| PtMode | Type of place. can for example be "bus" or "rail". |

```xml
					<ojp:Place>
						<ojp:StopPlace>
							<ojp:StopPlaceRef>1300033</ojp:StopPlaceRef>
							<ojp:StopPlaceName>
								<ojp:Text xml:lang="de">Aosta, Autostazione</ojp:Text>
							</ojp:StopPlaceName>
							<ojp:PrivateCode>
								<ojp:System>EFA</ojp:System>
								<ojp:Value>173775</ojp:Value>
							</ojp:PrivateCode>
							<ojp:PrivateCode>
								<ojp:System>LA-ExchangePoint-ID</ojp:System>
								<ojp:Value>it:itc:StopPlace:ITRP-ST0070039001:</ojp:Value>
							</ojp:PrivateCode>
							<ojp:TopographicPlaceRef>22007003:1</ojp:TopographicPlaceRef>
						</ojp:StopPlace>
						<ojp:LocationName>
							<ojp:Text xml:lang="de">Aosta</ojp:Text>
						</ojp:LocationName>
						<ojp:GeoPosition>
							<siri:Longitude>7.32432</siri:Longitude>
							<siri:Latitude>45.73516</siri:Latitude>
						</ojp:GeoPosition>
						<ojp:Extension>
							<ojp:PlaceExtensionStructure>
								<ojp:ExchangePoint>
									<ojp:WaitingTime>PT5M</ojp:WaitingTime>
								</ojp:ExchangePoint>
							</ojp:PlaceExtensionStructure>
						</ojp:Extension>
					</ojp:Place>
					<ojp:BorderPoint>false</ojp:BorderPoint>
					<ojp:Mode>
						<ojp:PtMode>bus</ojp:PtMode>
						<siri:BusSubmode>localBusService</siri:BusSubmode>
					</ojp:Mode>
				</ojp:Place>
```
