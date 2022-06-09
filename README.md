# swift_rust_test

This module is to test calling Swift functionality from Rust. It's organised into 2 parts:

## `src`: Rust Code

`build.rs` is using `swift_rs::build` to build and link the Swift executable. This is really nifty because it will automatically build teh Swift code during Rust build so you don't have to worry about code bases getting out of sync.

`main.rs` contains an `extern "C"` section to reference external code as C. The function name should match the name declaired in Swift (similar to how Tauri does it for JS/TS). You then call that (unsafe) function in the main function and you can see the result.

## `tauri-swift`: Swift Code

A Swift package complete with all the Xcode crud that comes along. Main bit to pay attention to is `Package.swift` which is what `swift_rs::build` uses to target and compile the code.

`Sources/tauri-swift/tauri-swift.swift` (lots of Swift there) contains the actual code.

You can see how the function is declared public and decorated with `@_cdecl("test")`. This is the name used in the C headings and is what should be referenced in Rust.

## Future Notes

I haven't tested this with any Apple API's yet, so I'm not sure how well this will interact with them. I'm planning to do a bit of testing using the User Notifications framework.

Swift does export an AST. This could be potentially used to create a Rust macro to derive the exposed C functions instead of having to explicitly define them in Rust.

There are a ton of issues on which types can be bridged through C which is the biggest hurdle.

I'm not exaclty sure how asyncronous code works/could be handled with this approach.
