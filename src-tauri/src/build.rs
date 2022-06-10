use swift_rs::build;

fn main() {
    build::link_swift();
    build::link_swift_package("tauri-swift", "../tauri-swift/");
}
