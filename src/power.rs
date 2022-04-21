use regex::Regex;
use serde::Serialize;
use std::process::Command;
use std::str;

/// Parses `pmset -g batt` output and returns the current MacBook power mode
pub fn get() -> PowerMode {
    lazy_static! {
        static ref RE: Regex = Regex::new(r".*'(?P<mode>.*) Power'\n.*\t(?P<percent>.*)%; (?P<status>.*); (?P<remaining>.*) present").unwrap();
    }

    let output = Command::new("pmset")
                .args(["-g", "batt"])
                .output()
                .expect("failed to execute process");

    let pmset = str::from_utf8(&output.stdout).unwrap();

    let caps = RE.captures(pmset).unwrap();

    PowerMode {
        isOnAC: true,
        isOnBattery: false,
        isCharged: false,
        chargingStatus: String::from(&caps["status"]),
        chargePercent: 50,
        remainingChargeTime: String::from(&caps["remaining"]),
    }
}


#[allow(non_snake_case)]
#[derive(Serialize)]
pub struct PowerMode {
    isOnAC: bool,
    isOnBattery: bool,
    isCharged: bool,
    chargingStatus: String,
    chargePercent: u16,
    remainingChargeTime: String,
}
