import SwiftRs
import Foundation
import UserNotifications

@_cdecl("swift_test")
public func test() -> SRData {
    
    // Convert to a string via Codable
    // Figure out how to pass those strings to Rust
    // Decode them using Serde/serde-json in Rust
    
    let container = Container(
        passedBool: true,
        passedInt: 42,
        passedNegativeInt: -42,
        passedString: "I'm a test string",
        passedEnum: .test(embeddedValue: 100)
    )
    
    let data = try! JSONEncoder().encode(container)
    let array = Array(data)
    
    
    return SRData(array)
}

@_cdecl("is_permission_granted")
public func isPermissionGranted() -> SRData {
    let center = UNUserNotificationCenter.current()
    
//    center.getNotificationSettings { settings in
//        print(settings.authorizationStatus)
//    }
    
    let data = try! JSONEncoder().encode(true)
    let array = Array(data)
    
    return SRData(array)
}

public struct Container: Codable {
    let passedBool: Bool
    let passedInt: Int
    let passedNegativeInt: Int
    let passedString: String
    let passedEnum: CustomEnum
}

public enum CustomEnum: Codable {
    case test(embeddedValue: Int)
}
