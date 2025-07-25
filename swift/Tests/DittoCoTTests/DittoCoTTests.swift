import XCTest
@testable import DittoCoTCore

final class DittoCoTTests: XCTestCase {
    func testDocumentTypeCreation() {
        let doc = ApiDocument(
            _id: "test-id",
            _c: 1,
            _r: false,
            a: "peer-key",
            b: 12345.0,
            d: "test-uid",
            e: "test-callsign",
            contentType: "application/json",
            data: "test-data",
            isFile: false,
            isRemoved: false,
            mime: "application/json",
            source: "test-source",
            tag: "test-tag",
            timeMillis: 1234567890,
            title: "Test Title"
        )
        
        XCTAssertEqual(doc.type, "api")
        XCTAssertEqual(doc._id, "test-id")
        XCTAssertEqual(doc._c, 1)
        XCTAssertEqual(doc._v, 2)
        XCTAssertEqual(doc._r, false)
    }
    
    func testJSONValueEquality() {
        let value1 = JSONValue.string("test")
        let value2 = JSONValue.string("test")
        let value3 = JSONValue.number(123.0)
        
        XCTAssertEqual(value1, value2)
        XCTAssertNotEqual(value1, value3)
    }
    
    func testJSONValueFromDict() {
        let dict: [String: Any] = ["key": "value", "number": 42]
        let jsonValue = JSONValue(dict)
        
        if case .object(let obj) = jsonValue {
            XCTAssertEqual(obj["key"], JSONValue.string("value"))
            XCTAssertEqual(obj["number"], JSONValue.number(42.0))
        } else {
            XCTFail("Expected object JSONValue")
        }
    }
}