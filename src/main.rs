use swift_rs::SRData;

fn main() {
    unsafe {
        objc_autoreleasePoolPush();
        // Commenting out the below line will show everything is working as expected
        test_struct();
        objc_autoreleasePoolPop();
    };
}

extern "C" {
    fn objc_autoreleasePoolPush();
    #[allow(dead_code)]
    fn test_struct() -> SRData;
    fn objc_autoreleasePoolPop();
}
