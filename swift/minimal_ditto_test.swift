#!/usr/bin/env swift

import Foundation
import DittoSwift

// Create a minimal test to see if the DQL error is in the Ditto SDK itself

print("🔬 MINIMAL DITTO TEST: Starting...")

// Load environment
guard let appId = ProcessInfo.processInfo.environment["DITTO_APP_ID"],
      let sharedKey = ProcessInfo.processInfo.environment["DITTO_SHARED_KEY"],
      let licenseToken = ProcessInfo.processInfo.environment["DITTO_LICENSE_TOKEN"] else {
    print("❌ Missing environment variables")
    exit(1)
}

print("🔬 Creating unique persistence directory...")
let documentsPath = FileManager.default.urls(for: .documentDirectory, in: .userDomainMask).first!
let timestamp = Int(Date().timeIntervalSince1970)
let randomID = Int.random(in: 1000...9999)
let persistenceDir = documentsPath.appendingPathComponent("MinimalDittoTest_\(timestamp)_\(randomID)")

print("🔬 Initializing Ditto...")
do {
    let ditto = Ditto(
        identity: .sharedKey(
            appID: appId,
            sharedKey: sharedKey,
            siteID: UInt64.random(in: 1...UInt64.max)
        ),
        persistenceDirectory: persistenceDir
    )
    
    print("🔬 Setting license token...")
    try ditto.setOfflineOnlyLicenseToken(licenseToken)
    
    print("🔬 Starting sync...")
    try ditto.startSync()
    
    print("🔬 Testing basic collection access...")
    let collection = ditto.store.collection("test")
    
    print("🔬 Testing simple query...")
    let docs = collection.find("_r != true").exec()
    print("🔬 SUCCESS: Found \(docs.count) documents")
    
    print("🔬 Testing subscription...")
    let subscription = collection.find("_r != true").subscribe()
    print("🔬 SUCCESS: Subscription created")
    
    print("🔬 All tests passed! Ditto SDK is working correctly.")
    
} catch {
    print("❌ MINIMAL TEST FAILED: \(error)")
    exit(1)
}