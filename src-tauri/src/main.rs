#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use serde::Deserialize;
use swift_rs::types::SRString;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![test, notification_test])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[allow(dead_code)]
#[tauri::command]
fn test() -> String {
    println!("I'm printing from Rust!");
    let sr_result = unsafe { swift_test() };
    // let string_result = sr_result.to_string();
    // let json: Container = serde_json::from_str(&string_result).expect("Couldn't parse");
    // println!(
    //     "Here is my result that I received from the Swift function: {:#?}",
    //     json
    // );

    // let formatted = format!("{:?}", json);

    return "Test".to_string();
}

#[tauri::command]
fn notification_test() -> String {
    let result = unsafe { is_permission_granted() };

    return result.to_string();
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

extern "C" {
    fn swift_test() -> SRString;
    fn is_permission_granted() -> SRString;
}
