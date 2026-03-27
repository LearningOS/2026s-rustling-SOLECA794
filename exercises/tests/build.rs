//! This is the build script for both tests7 and tests8.
//! You should modify this file to make both exercises pass.

fn main() {
    // 用于 tests7：设置环境变量 TEST_FOO
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let your_command = format!(
        "rustc-env=TEST_FOO={}",
        timestamp
    );
    println!("cargo:{}", your_command);

    // 用于 tests8：启用 "pass" 功能
    let your_command = "rustc-cfg=feature=\"pass\"";
    println!("cargo:{}", your_command);
}