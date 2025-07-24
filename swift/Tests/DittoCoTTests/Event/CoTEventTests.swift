import XCTest
@testable import DittoCoTCore

final class CoTEventTests: XCTestCase {
    
    // MARK: - Test Data
    
    private var testDate: Date {
        Date(timeIntervalSince1970: 1704067200) // 2024-01-01 00:00:00 UTC
    }
    
    private var testPoint: CoTPoint {
        CoTPoint(lat: 34.12345, lon: -118.12345, hae: 150.0, ce: 10.0, le: 5.0)
    }
    
    // MARK: - Initialization Tests
    
    func testInitializationWithRequiredFields() {
        let event = CoTEvent(
            uid: "TEST-123",
            type: "a-f-G-U-C",
            time: testDate,
            start: testDate,
            stale: testDate.addingTimeInterval(300),
            how: "m-g",
            point: testPoint
        )
        
        XCTAssertEqual(event.version, "2.0")
        XCTAssertEqual(event.uid, "TEST-123")
        XCTAssertEqual(event.type, "a-f-G-U-C")
        XCTAssertEqual(event.time, testDate)
        XCTAssertEqual(event.start, testDate)
        XCTAssertEqual(event.stale, testDate.addingTimeInterval(300))
        XCTAssertEqual(event.how, "m-g")
        XCTAssertEqual(event.point, testPoint)
        
        // Optional fields should be nil
        XCTAssertNil(event.access)
        XCTAssertNil(event.qos)
        XCTAssertNil(event.opex)
        XCTAssertNil(event.caveat)
        XCTAssertNil(event.releasableTo)
        XCTAssertNil(event.detail)
    }
    
    func testInitializationWithAllFields() {
        let detail = CoTDetail(["contact": ["callsign": "ALPHA-1"]])
        
        let event = CoTEvent(
            version: "2.1",
            uid: "TEST-456",
            type: "a-h-G",
            time: testDate,
            start: testDate,
            stale: testDate.addingTimeInterval(600),
            how: "h-g-i-g-o",
            point: testPoint,
            access: "nato",
            qos: "5-r-d",
            opex: "e-EXERCISE",
            caveat: "FOUO",
            releasableTo: "USA NATO",
            detail: detail
        )
        
        XCTAssertEqual(event.version, "2.1")
        XCTAssertEqual(event.access, "nato")
        XCTAssertEqual(event.qos, "5-r-d")
        XCTAssertEqual(event.opex, "e-EXERCISE")
        XCTAssertEqual(event.caveat, "FOUO")
        XCTAssertEqual(event.releasableTo, "USA NATO")
        XCTAssertNotNil(event.detail)
        XCTAssertEqual(event.callsign, "ALPHA-1")
    }
    
    // MARK: - Helper Method Tests
    
    func testIsValid() {
        let now = Date()
        let validEvent = CoTEvent(
            uid: "TEST-1",
            type: "a-f-G",
            time: now,
            start: now.addingTimeInterval(-60),
            stale: now.addingTimeInterval(60),
            how: "m-g",
            point: testPoint
        )
        
        let staleEvent = CoTEvent(
            uid: "TEST-2",
            type: "a-f-G",
            time: now,
            start: now.addingTimeInterval(-120),
            stale: now.addingTimeInterval(-60),
            how: "m-g",
            point: testPoint
        )
        
        XCTAssertTrue(validEvent.isValid)
        XCTAssertFalse(staleEvent.isValid)
    }
    
    func testIsStale() {
        let now = Date()
        let freshEvent = CoTEvent(
            uid: "TEST-1",
            type: "a-f-G",
            time: now,
            start: now,
            stale: now.addingTimeInterval(60),
            how: "m-g",
            point: testPoint
        )
        
        let staleEvent = CoTEvent(
            uid: "TEST-2",
            type: "a-f-G",
            time: now,
            start: now.addingTimeInterval(-120),
            stale: now.addingTimeInterval(-60),
            how: "m-g",
            point: testPoint
        )
        
        XCTAssertFalse(freshEvent.isStale)
        XCTAssertTrue(staleEvent.isStale)
    }
    
    func testTypeComponents() {
        let event = CoTEvent(
            uid: "TEST",
            type: "a-f-G-U-C-I",
            time: testDate,
            start: testDate,
            stale: testDate.addingTimeInterval(300),
            how: "m-g",
            point: testPoint
        )
        
        XCTAssertEqual(event.typeComponents, ["a", "f", "G", "U", "C", "I"])
        XCTAssertEqual(event.primaryType, "a")
        XCTAssertTrue(event.isAtom)
    }
    
    func testNonAtomType() {
        let event = CoTEvent(
            uid: "TEST",
            type: "b-m-p-s-p-i",
            time: testDate,
            start: testDate,
            stale: testDate.addingTimeInterval(300),
            how: "m-g",
            point: testPoint
        )
        
        XCTAssertEqual(event.primaryType, "b")
        XCTAssertFalse(event.isAtom)
    }
    
    // MARK: - Equatable Tests
    
    func testEquality() {
        let event1 = CoTEvent(
            uid: "TEST-123",
            type: "a-f-G-U-C",
            time: testDate,
            start: testDate,
            stale: testDate.addingTimeInterval(300),
            how: "m-g",
            point: testPoint
        )
        
        let event2 = CoTEvent(
            uid: "TEST-123",
            type: "a-f-G-U-C",
            time: testDate,
            start: testDate,
            stale: testDate.addingTimeInterval(300),
            how: "m-g",
            point: testPoint
        )
        
        XCTAssertEqual(event1, event2)
    }
    
    func testInequality() {
        let baseEvent = CoTEvent(
            uid: "TEST-123",
            type: "a-f-G-U-C",
            time: testDate,
            start: testDate,
            stale: testDate.addingTimeInterval(300),
            how: "m-g",
            point: testPoint
        )
        
        let differentUID = CoTEvent(
            uid: "TEST-456",
            type: "a-f-G-U-C",
            time: testDate,
            start: testDate,
            stale: testDate.addingTimeInterval(300),
            how: "m-g",
            point: testPoint
        )
        
        let differentType = CoTEvent(
            uid: "TEST-123",
            type: "a-h-G",
            time: testDate,
            start: testDate,
            stale: testDate.addingTimeInterval(300),
            how: "m-g",
            point: testPoint
        )
        
        XCTAssertNotEqual(baseEvent, differentUID)
        XCTAssertNotEqual(baseEvent, differentType)
    }
    
    // MARK: - Codable Tests
    
    func testEncodeDecode() throws {
        let detail = CoTDetail([
            "contact": ["callsign": "BRAVO-6"],
            "remarks": "Test event"
        ])
        
        let original = CoTEvent(
            uid: "TEST-789",
            type: "a-f-G-U-C",
            time: testDate,
            start: testDate,
            stale: testDate.addingTimeInterval(300),
            how: "m-g",
            point: testPoint,
            access: "unrestricted",
            qos: "1-r-c",
            detail: detail
        )
        
        let encoder = JSONEncoder()
        encoder.dateEncodingStrategy = .iso8601
        let data = try encoder.encode(original)
        
        let decoder = JSONDecoder()
        decoder.dateDecodingStrategy = .iso8601
        let decoded = try decoder.decode(CoTEvent.self, from: data)
        
        XCTAssertEqual(original, decoded)
        XCTAssertEqual(decoded.callsign, "BRAVO-6")
    }
    
    // MARK: - CustomStringConvertible Tests
    
    func testDescription() {
        let event = CoTEvent(
            uid: "DESC-TEST",
            type: "a-f-G",
            time: testDate,
            start: testDate,
            stale: testDate.addingTimeInterval(300),
            how: "m-g",
            point: testPoint
        )
        
        let description = event.description
        
        XCTAssertTrue(description.contains("DESC-TEST"))
        XCTAssertTrue(description.contains("a-f-G"))
        XCTAssertTrue(description.contains("34.12345"))
        XCTAssertTrue(description.contains("-118.12345"))
    }
    
    // MARK: - Complex Scenario Tests
    
    func testMilitaryUnitEvent() {
        let detail = CoTDetail([
            "contact": [
                "callsign": "EAGLE-1",
                "dsn": "312-555-0123",
                "email": "eagle1@mil.example"
            ],
            "group": [
                "name": "1st Platoon",
                "role": "Team Lead"
            ],
            "_flow-tags_": [
                "ATAK": ["TAK-Server-1"]
            ],
            "remarks": "On patrol, sector clear"
        ])
        
        let event = CoTEvent(
            uid: "MIL-UNIT-001",
            type: "a-f-G-U-C-I",
            time: Date(),
            start: Date(),
            stale: Date().addingTimeInterval(3600),
            how: "m-g",
            point: CoTPoint(lat: 33.5, lon: -117.2, hae: 25.0, ce: 5.0, le: 2.0),
            access: "coalition",
            qos: "3-r-d",
            opex: "o-OPERATION-FREEDOM",
            caveat: "FOUO",
            releasableTo: "USA GBR CAN AUS NZL",
            detail: detail
        )
        
        XCTAssertEqual(event.callsign, "EAGLE-1")
        XCTAssertEqual(event.detail?.getValue(at: "group.name"), .string("1st Platoon"))
        XCTAssertEqual(event.opex, "o-OPERATION-FREEDOM")
    }
    
    // MARK: - XML Serialization Tests
    
    func testXMLSerialization() throws {
        let event = CoTEvent(
            uid: "XML-TEST-123",
            type: "a-f-G-U-C",
            time: testDate,
            start: testDate,
            stale: testDate.addingTimeInterval(300),
            how: "m-g",
            point: testPoint,
            access: "unrestricted",
            qos: "1-r-c"
        )
        
        let xml = try event.toXML()
        
        XCTAssertTrue(xml.contains("<event"))
        XCTAssertTrue(xml.contains("uid=\"XML-TEST-123\""))
        XCTAssertTrue(xml.contains("type=\"a-f-G-U-C\""))
        XCTAssertTrue(xml.contains("<point"))
        XCTAssertTrue(xml.contains("lat=\"34.12345\""))
        XCTAssertTrue(xml.contains("lon=\"-118.12345\""))
    }
    
    func testXMLDeserialization() throws {
        let xmlString = """
        <event version="2.0" uid="XML-PARSE-TEST" type="a-f-G" time="2024-01-01T00:00:00.000Z" start="2024-01-01T00:00:00.000Z" stale="2024-01-01T00:05:00.000Z" how="m-g">
            <point lat="34.12345" lon="-118.12345" hae="150.0" ce="10.0" le="5.0"/>
        </event>
        """
        
        let event = try CoTEvent.fromXML(xmlString)
        
        XCTAssertEqual(event.uid, "XML-PARSE-TEST")
        XCTAssertEqual(event.type, "a-f-G")
        XCTAssertEqual(event.point.lat, 34.12345)
        XCTAssertEqual(event.point.lon, -118.12345)
        XCTAssertEqual(event.point.hae, 150.0)
    }
    
    func testXMLRoundTrip() throws {
        let original = CoTEvent(
            uid: "ROUNDTRIP-TEST",
            type: "a-f-G-U-C",
            time: testDate,
            start: testDate,
            stale: testDate.addingTimeInterval(300),
            how: "m-g",
            point: testPoint,
            access: "coalition",
            qos: "2-r-c"
        )
        
        let xml = try original.toXML()
        let parsed = try CoTEvent.fromXML(xml)
        
        XCTAssertEqual(original.uid, parsed.uid)
        XCTAssertEqual(original.type, parsed.type)
        XCTAssertEqual(original.point, parsed.point)
        XCTAssertEqual(original.access, parsed.access)
        XCTAssertEqual(original.qos, parsed.qos)
    }
    
    func testXMLPrettyPrint() throws {
        let event = CoTEvent(
            uid: "PRETTY-TEST",
            type: "a-f-G",
            time: testDate,
            start: testDate,
            stale: testDate.addingTimeInterval(300),
            how: "m-g",
            point: testPoint
        )
        
        let prettyXML = try event.toXML(prettyPrint: true)
        let compactXML = try event.toXML(prettyPrint: false)
        
        XCTAssertTrue(prettyXML.contains("\n"))
        XCTAssertFalse(compactXML.contains("\n"))
        XCTAssertGreaterThan(prettyXML.count, compactXML.count)
    }
    
    func testXMLWithoutDetail() throws {
        // Note: Detail support is not implemented in basic XML serialization
        let event = CoTEvent(
            uid: "NO-DETAIL-XML-TEST",
            type: "a-f-G-U-C",
            time: testDate,
            start: testDate,
            stale: testDate.addingTimeInterval(300),
            how: "m-g",
            point: testPoint
        )
        
        let xml = try event.toXML(prettyPrint: true)
        let parsed = try CoTEvent.fromXML(xml)
        
        XCTAssertEqual(parsed.uid, "NO-DETAIL-XML-TEST")
        XCTAssertEqual(parsed.type, "a-f-G-U-C")
        XCTAssertNil(parsed.detail) // Detail is not supported in basic XML
    }
    
    func testInvalidXMLHandling() {
        let invalidXML = "<invalid>not a cot event</invalid>"
        
        XCTAssertThrowsError(try CoTEvent.fromXML(invalidXML)) { error in
            XCTAssertTrue(error is DecodingError)
        }
    }
    
    // MARK: - Swift Idiomatic Features Tests
    
    func testStaticFactoryMethods() {
        let point = CoTPoint(lat: 40.0, lon: -74.0)
        
        let blueForce = CoTEvent.blueForceTrack(uid: "BF-123", at: point, callsign: "ALPHA-1")
        XCTAssertEqual(blueForce.uid, "BF-123")
        XCTAssertEqual(blueForce.type, "a-f-G")
        XCTAssertEqual(blueForce.callsign, "ALPHA-1")
        
        let emergency = CoTEvent.emergency(uid: "EMRG-456", at: point, message: "Help needed")
        XCTAssertEqual(emergency.uid, "EMRG-456")
        XCTAssertEqual(emergency.type, "b-a-o-tbl")
        XCTAssertEqual(emergency.detail?.remarks, "Help needed")
    }
    
    func testResultBasedBuilder() {
        let point = CoTPoint(lat: 34.0, lon: -118.0)
        
        // Test successful build
        let successResult = CoTEvent.builder()
            .uid("TEST-SUCCESS")
            .type("a-f-G")
            .point(point)
            .buildResult()
        
        switch successResult {
        case .success(let event):
            XCTAssertEqual(event.uid, "TEST-SUCCESS")
        case .failure:
            XCTFail("Expected success")
        }
        
        // Test failure build
        let failureResult = CoTEvent.builder()
            .uid("TEST-FAILURE")
            // Missing type and point
            .buildResult()
        
        switch failureResult {
        case .success:
            XCTFail("Expected failure")
        case .failure(let error):
            XCTAssertTrue(error.localizedDescription.contains("Missing required field"))
        }
    }
    
    func testEventValidation() {
        let validEvent = CoTEvent(
            uid: "VALID-123",
            type: "a-f-G-U-C",
            time: testDate,
            start: testDate,
            stale: testDate.addingTimeInterval(300),
            how: "m-g",
            point: CoTPoint(lat: 45.0, lon: -90.0)
        )
        
        XCTAssertTrue(validEvent.isValidEvent)
        
        switch validEvent.validationResult {
        case .success:
            // Expected
            break
        case .failure(let error):
            XCTFail("Validation should succeed: \(error)")
        }
        
        // Test invalid version (since CoTPoint prevents invalid coordinates at creation)
        let invalidVersionEvent = CoTEvent(
            version: "1.0", // Invalid version
            uid: "INVALID-123",
            type: "a-f-G",
            time: testDate,
            start: testDate,
            stale: testDate.addingTimeInterval(300),
            how: "m-g",
            point: CoTPoint(lat: 45.0, lon: -90.0)
        )
        
        XCTAssertFalse(invalidVersionEvent.isValidEvent)
        
        switch invalidVersionEvent.validationResult {
        case .success:
            XCTFail("Validation should fail")
        case .failure(let error):
            XCTAssertTrue(error.localizedDescription.contains("Invalid CoT version"))
        }
        
        // Test invalid type format
        let invalidTypeEvent = CoTEvent(
            uid: "INVALID-TYPE",
            type: "invalid_type!", // Invalid characters
            time: testDate,
            start: testDate,
            stale: testDate.addingTimeInterval(300),
            how: "m-g",
            point: CoTPoint(lat: 45.0, lon: -90.0)
        )
        
        XCTAssertFalse(invalidTypeEvent.isValidEvent)
        
        switch invalidTypeEvent.validationResult {
        case .success:
            XCTFail("Validation should fail")
        case .failure(let error):
            XCTAssertTrue(error.localizedDescription.contains("Invalid event type format"))
        }
    }
    
    // MARK: - Performance Tests
    
    func testCreationPerformance() {
        measure {
            for i in 0..<1000 {
                _ = CoTEvent(
                    uid: "PERF-\(i)",
                    type: "a-f-G-U-C",
                    time: Date(),
                    start: Date(),
                    stale: Date().addingTimeInterval(300),
                    how: "m-g",
                    point: testPoint
                )
            }
        }
    }
    
    func testCodablePerformance() throws {
        let event = CoTEvent(
            uid: "PERF-TEST",
            type: "a-f-G-U-C",
            time: testDate,
            start: testDate,
            stale: testDate.addingTimeInterval(300),
            how: "m-g",
            point: testPoint,
            detail: CoTDetail(["test": "data"])
        )
        
        let encoder = JSONEncoder()
        let decoder = JSONDecoder()
        
        measure {
            for _ in 0..<1000 {
                let data = try! encoder.encode(event)
                _ = try! decoder.decode(CoTEvent.self, from: data)
            }
        }
    }
    
    func testXMLPerformance() throws {
        let event = CoTEvent(
            uid: "XML-PERF-TEST",
            type: "a-f-G-U-C",
            time: testDate,
            start: testDate,
            stale: testDate.addingTimeInterval(300),
            how: "m-g",
            point: testPoint,
            detail: CoTDetail(["test": "data"])
        )
        
        measure {
            for _ in 0..<100 {
                let xml = try! event.toXML()
                _ = try! CoTEvent.fromXML(xml)
            }
        }
    }
}