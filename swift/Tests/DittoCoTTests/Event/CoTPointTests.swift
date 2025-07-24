import XCTest
@testable import DittoCoTCore

final class CoTPointTests: XCTestCase {
    
    // MARK: - Initialization Tests
    
    func testInitializationWithDefaultValues() {
        let point = CoTPoint(lat: 34.12345, lon: -118.12345)
        
        XCTAssertEqual(point.lat, 34.12345)
        XCTAssertEqual(point.lon, -118.12345)
        XCTAssertEqual(point.hae, 0.0)
        XCTAssertEqual(point.ce, 999999.0)
        XCTAssertEqual(point.le, 999999.0)
    }
    
    func testInitializationWithAllValues() {
        let point = CoTPoint(lat: 34.12345, lon: -118.12345, hae: 150.0, ce: 10.0, le: 5.0)
        
        XCTAssertEqual(point.lat, 34.12345)
        XCTAssertEqual(point.lon, -118.12345)
        XCTAssertEqual(point.hae, 150.0)
        XCTAssertEqual(point.ce, 10.0)
        XCTAssertEqual(point.le, 5.0)
    }
    
    // MARK: - Boundary Tests
    
    func testValidLatitudeBoundaries() {
        // Test minimum latitude
        let minPoint = CoTPoint(lat: -90.0, lon: 0.0)
        XCTAssertEqual(minPoint.lat, -90.0)
        
        // Test maximum latitude
        let maxPoint = CoTPoint(lat: 90.0, lon: 0.0)
        XCTAssertEqual(maxPoint.lat, 90.0)
    }
    
    func testValidLongitudeBoundaries() {
        // Test minimum longitude
        let minPoint = CoTPoint(lat: 0.0, lon: -180.0)
        XCTAssertEqual(minPoint.lon, -180.0)
        
        // Test maximum longitude
        let maxPoint = CoTPoint(lat: 0.0, lon: 180.0)
        XCTAssertEqual(maxPoint.lon, 180.0)
    }
    
    // MARK: - Precondition Tests
    // Note: These tests will trap in debug builds, so they're commented out
    // In production, you might want to use throwing initializers instead
    
    /*
    func testInvalidLatitudePrecondition() {
        // This would trap: CoTPoint(lat: 91.0, lon: 0.0)
        // This would trap: CoTPoint(lat: -91.0, lon: 0.0)
    }
    
    func testInvalidLongitudePrecondition() {
        // This would trap: CoTPoint(lat: 0.0, lon: 181.0)
        // This would trap: CoTPoint(lat: 0.0, lon: -181.0)
    }
    
    func testInvalidErrorValuesPrecondition() {
        // This would trap: CoTPoint(lat: 0.0, lon: 0.0, hae: 0.0, ce: -1.0, le: 5.0)
        // This would trap: CoTPoint(lat: 0.0, lon: 0.0, hae: 0.0, ce: 5.0, le: -1.0)
    }
    */
    
    // MARK: - Equatable Tests
    
    func testEquality() {
        let point1 = CoTPoint(lat: 34.12345, lon: -118.12345, hae: 150.0, ce: 10.0, le: 5.0)
        let point2 = CoTPoint(lat: 34.12345, lon: -118.12345, hae: 150.0, ce: 10.0, le: 5.0)
        
        XCTAssertEqual(point1, point2)
    }
    
    func testInequality() {
        let point1 = CoTPoint(lat: 34.12345, lon: -118.12345)
        let point2 = CoTPoint(lat: 34.12346, lon: -118.12345)
        let point3 = CoTPoint(lat: 34.12345, lon: -118.12346)
        let point4 = CoTPoint(lat: 34.12345, lon: -118.12345, hae: 150.0)
        
        XCTAssertNotEqual(point1, point2)
        XCTAssertNotEqual(point1, point3)
        XCTAssertNotEqual(point1, point4)
    }
    
    // MARK: - Codable Tests
    
    func testEncodeDecode() throws {
        let original = CoTPoint(lat: 34.12345, lon: -118.12345, hae: 150.0, ce: 10.0, le: 5.0)
        
        let encoder = JSONEncoder()
        let data = try encoder.encode(original)
        
        let decoder = JSONDecoder()
        let decoded = try decoder.decode(CoTPoint.self, from: data)
        
        XCTAssertEqual(original, decoded)
    }
    
    func testJSONRepresentation() throws {
        let point = CoTPoint(lat: 34.12345, lon: -118.12345, hae: 150.0, ce: 10.0, le: 5.0)
        
        let encoder = JSONEncoder()
        encoder.outputFormatting = [.sortedKeys, .prettyPrinted]
        let data = try encoder.encode(point)
        let json = String(data: data, encoding: .utf8)!
        
        XCTAssertTrue(json.contains("\"lat\" : 34.12345"))
        XCTAssertTrue(json.contains("\"lon\" : -118.12345"))
        XCTAssertTrue(json.contains("\"hae\" : 150"))
        XCTAssertTrue(json.contains("\"ce\" : 10"))
        XCTAssertTrue(json.contains("\"le\" : 5"))
    }
    
    // MARK: - CustomStringConvertible Tests
    
    func testDescription() {
        let point = CoTPoint(lat: 34.12345, lon: -118.12345, hae: 150.0, ce: 10.0, le: 5.0)
        let description = point.description
        
        XCTAssertTrue(description.contains("34.12345"))
        XCTAssertTrue(description.contains("-118.12345"))
        XCTAssertTrue(description.contains("150.0"))
        XCTAssertTrue(description.contains("10.0"))
        XCTAssertTrue(description.contains("5.0"))
    }
    
    // MARK: - Performance Tests
    
    func testInitializationPerformance() {
        measure {
            for _ in 0..<10000 {
                _ = CoTPoint(lat: 34.12345, lon: -118.12345, hae: 150.0, ce: 10.0, le: 5.0)
            }
        }
    }
    
    func testCodablePerformance() throws {
        let point = CoTPoint(lat: 34.12345, lon: -118.12345, hae: 150.0, ce: 10.0, le: 5.0)
        let encoder = JSONEncoder()
        let decoder = JSONDecoder()
        
        measure {
            for _ in 0..<1000 {
                let data = try! encoder.encode(point)
                _ = try! decoder.decode(CoTPoint.self, from: data)
            }
        }
    }
}