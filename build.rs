use swift_rs::build;

fn main() {
    println!("Building and linking Swift");
    build::link_swift();
    build::link_swift_package("tauri-swift", "tauri-swift/");
}
