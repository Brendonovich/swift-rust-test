use serde_json::Value;
use swift_rs::*;

swift_fn!(test_struct() -> SRData);

fn main() {
    let pool = unsafe { objc_autoreleasePoolPush() };

    let data = test_struct();
    let json = serde_json::from_slice::<Value>(&data).unwrap();
    println!("{}", serde_json::to_string_pretty(&json).unwrap());

    unsafe { objc_autoreleasePoolPop(pool) };
}

extern "C" {
    fn objc_autoreleasePoolPush() -> *mut std::ffi::c_void;
    fn objc_autoreleasePoolPop(pool: *mut std::ffi::c_void);
}
