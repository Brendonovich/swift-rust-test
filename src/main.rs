fn main() {
    let result = unsafe { test() };
    println!("Hello, world! Here is my result: {}", result);
}

extern "C" {
    fn test() -> bool;
}
