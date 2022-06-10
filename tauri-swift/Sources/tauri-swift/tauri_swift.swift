import SwiftRs
import Foundation
import UserNotifications

@_cdecl("swift_test")
public func test() -> SRString {
    print("I'm printing from Swift!")
    
    let container = Container(
        passedBool: true,
        passedInt: 42,
        passedNegativeInt: -42,
        passedString: "I'm a test string",
        passedEnum: .test(embeddedValue: 100)
    )
    
    let data = try! JSONEncoder().encode(container)
    let string = String(data: data, encoding: .utf8)!
    
    return SRString(string)
}

@_cdecl("is_permission_granted")
public func isPermissionGranted() -> SRString {
    let center = UNUserNotificationCenter.current()
    
    center.getNotificationSettings { settings in
        print(settings.authorizationStatus)
    }
    
    let data = try! JSONEncoder().encode(true)
    let string = String(data: data, encoding: .utf8)!
    
    return SRString(string)
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
