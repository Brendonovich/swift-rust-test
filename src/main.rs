fn main() {
    println!("I'm printing from Rust!");
    let result = unsafe { test() };
    println!(
        "Here is my result that I received from the Swift function: {}",
        result
    );
}

extern "C" {
    fn test() -> bool;
}
