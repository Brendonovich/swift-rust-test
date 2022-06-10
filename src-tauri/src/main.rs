#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use serde::Deserialize;
use swift_rs::SRData;

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

    unsafe {
        let result = swift_test();
        // let container: Container = serde_json::from_slice(&result).expect("Couldn't parse");
        // println!(
        //     "Here is my result that I received from the Swift function: {:#?}",
        //     container
        // );
    };

    // It seems like deallocation is messing something up

    return "Hello".to_string();
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
    fn swift_test() -> SRData;
    fn is_permission_granted() -> bool;
}
