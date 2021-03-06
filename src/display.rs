use std::process::Command;
use std::str::{self, FromStr};

/// Turns off MacBook display
pub fn off() {
    Command::new("pmset")
        .arg("displaysleepnow")
        .output()
        .expect("failed to execute process pmset");
}

/// Turns on MacBook display
pub fn on() {
    Command::new("caffeinate")
        .args(["-u", "-t", "1"])
        .output()
        .expect("failed to execute process caffeinate");
}

/// Reports MacBook display status
pub fn is_on() -> bool {
    let output = Command::new("sh")
        .args(["-c", "pmset -g powerstate | grep DCPDPDeviceProxy | wc -l"])
        .output()
        .expect("failed to execute process pmset");

    let stdout = str::from_utf8(&output.stdout).unwrap();

    i16::from_str(str::trim(stdout)).unwrap() > 0
}
