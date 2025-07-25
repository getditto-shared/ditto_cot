// swift-tools-version: 5.9
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "DittoCoT",
    platforms: [
        .iOS(.v15),
        .macOS(.v12),
        .watchOS(.v8),
        .tvOS(.v15)
    ],
    products: [
        // Products define the executables and libraries a package produces, making them visible to other packages.
        .library(
            name: "DittoCoT",
            targets: ["DittoCoT"]),
        .library(
            name: "DittoCoTCore",
            targets: ["DittoCoTCore"]),
        .executable(
            name: "CoTExampleApp",
            targets: ["CoTExampleApp"]),
    ],
    dependencies: [
        // Dependencies declare other packages that this package depends on.
        .package(url: "https://github.com/getditto/DittoSwiftPackage.git", from: "4.11.0"),
        .package(url: "https://github.com/CoreOffice/XMLCoder.git", from: "0.17.1"),
        .package(url: "https://github.com/apple/swift-argument-parser.git", from: "1.3.0"),
    ],
    targets: [
        // Targets are the basic building blocks of a package, defining a module or a test suite.
        // Targets can depend on other targets in this package and products from dependencies.
        .target(
            name: "DittoCoTCore",
            dependencies: [
                .product(name: "XMLCoder", package: "XMLCoder"),
            ],
            path: "Sources/DittoCoTCore"),
        .target(
            name: "DittoCoT",
            dependencies: [
                "DittoCoTCore",
                .product(name: "DittoSwift", package: "DittoSwiftPackage"),
            ],
            path: "Sources/DittoCoT"),
        .testTarget(
            name: "DittoCoTTests",
            dependencies: ["DittoCoTCore"],
            path: "Tests/DittoCoTTests"),
        .testTarget(
            name: "IntegrationTests",
            dependencies: ["DittoCoTCore"],
            path: "Tests/IntegrationTests"),
        .executableTarget(
            name: "ditto-cot-codegen",
            dependencies: [
                .product(name: "ArgumentParser", package: "swift-argument-parser"),
            ],
            path: "Sources/CodeGen"),
        .executableTarget(
            name: "CoTExampleApp",
            dependencies: [
                "DittoCoT",
                "DittoCoTCore",
                .product(name: "DittoSwift", package: "DittoSwiftPackage"),
            ],
            path: "Examples/SwiftUI"),
    ]
)