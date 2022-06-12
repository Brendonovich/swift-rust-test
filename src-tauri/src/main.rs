#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use serde::Deserialize;
use swift_rs::*;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            rust_test_bool,
            rust_test_int,
            rust_test_struct
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn rust_test_bool() -> String {
    let result = test_bool();
    return result.to_string();
}

#[tauri::command]
fn rust_test_int() -> String {
    let result = test_int();
    return result.to_string();
}

#[tauri::command]
fn rust_test_struct() -> String {
    println!("I'm printing from Rust");

    let result = test_struct();
    let container: Container = serde_json::from_slice(&result).expect("Couldn't parse");

    println!(
        "Here is my result that I received from the Swift function: {:#?}",
        container
    );

    let string_result = format!("{:?}", container);

    // It seems like deallocation is messing something up

    return string_result;
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
struct Container {
    passed_bool: bool,
    passed_int: u8,
    passed_negative_int: i8,
    passed_string: String,
    passed_enum: CustomEnum,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
enum CustomEnum {
    #[serde(rename_all = "camelCase")]
    Test { embedded_value: u8 },
}

swift_fn!(test_bool() -> Bool);
swift_fn!(test_int() -> Int);
swift_fn!(test_struct() -> SRData);
