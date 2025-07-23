import XCTest
@testable import DittoCoTCore

final class IntegrationTests: XCTestCase {
    func testUnionTypeDecoding() {
        // For now, just test that we can create the union type
        let apiDoc = ApiDocument(
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
        
        let unionDoc = DittoCoTDocument.api(apiDoc)
        
        switch unionDoc {
        case .api(let doc):
            XCTAssertEqual(doc.type, "api")
        default:
            XCTFail("Expected API document")
        }
    }
}