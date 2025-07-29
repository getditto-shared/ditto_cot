import Foundation
import DittoCoTCore

// Test what Swift thinks the ChatDocument initializer signature is
let test = ChatDocument(
    _id: "test",
    a: "test", 
    b: 123,  // This should be Int
    d: "test",
    _c: 0,
    _r: false,
    e: "test",
    authorCallsign: "test",
    authorType: "test", 
    authorUid: "test",
    g: "test",
    h: 0.0,
    i: 0.0,
    j: 0.0,
    k: 0.0,
    l: 0.0,
    location: "test",
    message: "test",
    n: 456,  // This should be Int
    o: 789,  // This should be Int
    p: "test",
    parent: "test",
    q: "test", 
    r: JSONValue([:]),
    room: "test",
    roomId: "test",
    s: "test",
    source: "test",
    t: "test",
    time: "test",
    u: "test",
    v: "test",
    w: "test"
)

print("Test completed successfully")