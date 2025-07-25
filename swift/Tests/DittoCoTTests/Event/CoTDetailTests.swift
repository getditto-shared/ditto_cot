import XCTest
@testable import DittoCoTCore

final class CoTDetailTests: XCTestCase {
    
    // MARK: - Initialization Tests
    
    func testInitializationEmpty() {
        let detail = CoTDetail()
        
        if case .object(let dict) = detail.value {
            XCTAssertTrue(dict.isEmpty)
        } else {
            XCTFail("Expected empty object")
        }
    }
    
    func testInitializationWithDictionary() {
        let dict: [String: Any] = [
            "contact": ["callsign": "ALPHA-1"],
            "remarks": "Test remarks"
        ]
        
        let detail = CoTDetail(dict)
        
        XCTAssertEqual(detail.callsign, "ALPHA-1")
        XCTAssertEqual(detail.remarks, "Test remarks")
    }
    
    func testInitializationWithJSONValue() {
        let jsonValue = JSONValue.object([
            "test": .string("value"),
            "nested": .object(["key": .number(42)])
        ])
        
        let detail = CoTDetail(jsonValue)
        
        XCTAssertEqual(detail.getValue(at: "test"), .string("value"))
        XCTAssertEqual(detail.getValue(at: "nested.key"), .number(42))
    }
    
    // MARK: - Get Value Tests
    
    func testGetValueSimplePath() {
        let detail = CoTDetail(["key": "value"])
        
        XCTAssertEqual(detail.getValue(at: "key"), .string("value"))
    }
    
    func testGetValueNestedPath() {
        let detail = CoTDetail([
            "level1": [
                "level2": [
                    "level3": "deep value"
                ]
            ]
        ])
        
        XCTAssertEqual(detail.getValue(at: "level1.level2.level3"), .string("deep value"))
    }
    
    func testGetValueNonExistentPath() {
        let detail = CoTDetail(["key": "value"])
        
        XCTAssertNil(detail.getValue(at: "nonexistent"))
        XCTAssertNil(detail.getValue(at: "key.nonexistent"))
    }
    
    func testGetValuePartialPath() {
        let detail = CoTDetail([
            "contact": ["callsign": "ALPHA-1", "phone": "555-1234"]
        ])
        
        if case .object(let dict) = detail.getValue(at: "contact") {
            XCTAssertEqual(dict["callsign"], .string("ALPHA-1"))
            XCTAssertEqual(dict["phone"], .string("555-1234"))
        } else {
            XCTFail("Expected object at contact path")
        }
    }
    
    // MARK: - Set Value Tests
    
    func testSetValueSimplePath() {
        let detail = CoTDetail()
        let updated = detail.settingValue(.string("test"), at: "key")
        
        XCTAssertEqual(updated.getValue(at: "key"), .string("test"))
    }
    
    func testSetValueNestedPath() {
        let detail = CoTDetail()
        let updated = detail.settingValue(.string("deep"), at: "level1.level2.level3")
        
        XCTAssertEqual(updated.getValue(at: "level1.level2.level3"), .string("deep"))
    }
    
    func testSetValueOverwriteExisting() {
        let detail = CoTDetail(["key": "old"])
        let updated = detail.settingValue(.string("new"), at: "key")
        
        XCTAssertEqual(updated.getValue(at: "key"), .string("new"))
    }
    
    func testSetValuePreservesOtherValues() {
        let detail = CoTDetail([
            "keep": "this",
            "nested": ["keep": "also"]
        ])
        
        let updated = detail
            .settingValue(.string("new"), at: "added")
            .settingValue(.string("updated"), at: "nested.changed")
        
        XCTAssertEqual(updated.getValue(at: "keep"), .string("this"))
        XCTAssertEqual(updated.getValue(at: "nested.keep"), .string("also"))
        XCTAssertEqual(updated.getValue(at: "added"), .string("new"))
        XCTAssertEqual(updated.getValue(at: "nested.changed"), .string("updated"))
    }
    
    // MARK: - Common Properties Tests
    
    func testCallsignProperty() {
        var detail = CoTDetail()
        XCTAssertNil(detail.callsign)
        
        detail = detail.settingValue(.string("BRAVO-2"), at: "contact.callsign")
        XCTAssertEqual(detail.callsign, "BRAVO-2")
    }
    
    func testRemarksProperty() {
        var detail = CoTDetail()
        XCTAssertNil(detail.remarks)
        
        detail = detail.settingValue(.string("Test remarks here"), at: "remarks")
        XCTAssertEqual(detail.remarks, "Test remarks here")
    }
    
    func testColorProperty() {
        var detail = CoTDetail()
        XCTAssertNil(detail.color)
        
        detail = detail.settingValue(.string("FF0000FF"), at: "color.argb")
        XCTAssertEqual(detail.color, "FF0000FF")
    }
    
    // MARK: - Codable Tests
    
    func testEncodeDecode() throws {
        let original = CoTDetail([
            "contact": ["callsign": "ALPHA-1"],
            "remarks": "Test",
            "battery": 85
        ])
        
        let encoder = JSONEncoder()
        let data = try encoder.encode(original)
        
        let decoder = JSONDecoder()
        let decoded = try decoder.decode(CoTDetail.self, from: data)
        
        XCTAssertEqual(decoded.callsign, "ALPHA-1")
        XCTAssertEqual(decoded.remarks, "Test")
        
        // Test simple field preservation
        XCTAssertEqual(decoded.getValue(at: "battery"), .number(85))
    }
    
    // MARK: - Complex Scenario Tests
    
    func testComplexDetailStructure() {
        let complexData: [String: Any] = [
            "contact": [
                "callsign": "EAGLE-1",
                "phone": "555-0123",
                "email": "eagle1@example.com"
            ],
            "status": [
                "battery": 85,
                "fuel": 0.75,
                "readiness": "green"
            ],
            "track": [
                "course": 270,
                "speed": 55.5,
                "history": [
                    ["lat": 34.1, "lon": -118.1, "time": "2024-01-01T12:00:00Z"],
                    ["lat": 34.2, "lon": -118.2, "time": "2024-01-01T12:05:00Z"]
                ]
            ],
            "remarks": "On patrol, all systems nominal"
        ]
        
        let detail = CoTDetail(complexData)
        
        // Test various paths
        XCTAssertEqual(detail.getValue(at: "contact.email"), .string("eagle1@example.com"))
        XCTAssertEqual(detail.getValue(at: "status.battery"), .number(85))
        XCTAssertEqual(detail.getValue(at: "status.fuel"), .number(0.75))
        XCTAssertEqual(detail.getValue(at: "track.course"), .number(270))
        
        // Test array access
        if case .array(let history) = detail.getValue(at: "track.history") {
            XCTAssertEqual(history.count, 2)
        } else {
            XCTFail("Expected array for track.history")
        }
    }
    
    // MARK: - Performance Tests
    
    func testGetValuePerformance() {
        let detail = CoTDetail([
            "level1": [
                "level2": [
                    "level3": [
                        "level4": [
                            "level5": "deep value"
                        ]
                    ]
                ]
            ]
        ])
        
        measure {
            for _ in 0..<10000 {
                _ = detail.getValue(at: "level1.level2.level3.level4.level5")
            }
        }
    }
    
    func testSetValuePerformance() {
        let detail = CoTDetail()
        
        measure {
            var current = detail
            for i in 0..<1000 {
                current = current.settingValue(.number(Double(i)), at: "path.to.value.\(i)")
            }
        }
    }
}