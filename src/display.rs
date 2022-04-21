use std::process::Command;
use std::str::{self, FromStr};

/// turn off MacBook display
pub fn off() {
    Command::new("pmset")
        .arg("displaysleepnow")
        .output()
        .expect("failed to execute process pmset");
}

/// turn on MacBook display
pub fn on() {
    Command::new("caffeinate")
        .args(["-u", "-t", "1"])
        .output()
        .expect("failed to execute process caffeinate");
}

pub fn is_on() -> bool {
    let output = Command::new("sh")
        .args(["-c", "pmset -g powerstate | grep DCPDPDeviceProxy | wc -l"])
        .output()
        .expect("failed to execute process pmset");

    i16::from_str(str::trim(str::from_utf8(&output.stdout).unwrap())).unwrap() > 0
}
