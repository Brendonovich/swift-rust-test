// swift-tools-version: 5.6
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "tauri-swift",
    platforms: [
        .macOS(.v11)
    ],
    products: [
        // Products define the executables and libraries a package produces, and make them visible to other packages.
        .library(
            name: "tauri-swift",
            type: .static,
            targets: ["tauri-swift"]),
    ],
    dependencies: [
        // Dependencies declare other packages that this package depends on.
       .package(url: "https://github.com/Brendonovich/swift-rs.git", from: "0.3.0"),
    ],
    targets: [
        // Targets are the basic building blocks of a package. A target can define a module or a test suite.
        // Targets can depend on other targets in this package, and on products in packages this package depends on.
        .target(
            name: "tauri-swift",
            dependencies: [.product(name: "SwiftRs", package: "swift-rs")]
        ),
    ]
)
