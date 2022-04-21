use regex::Regex;
use serde::Serialize;
use std::process::Command;
use std::str;
use std::str::FromStr;

/// Parses `pmset -g batt` output and returns the current MacBook power mode
pub fn get() -> PowerMode {
    lazy_static! {
        static ref RE: Regex = Regex::new(r".*'(?P<mode>.*) Power'\n.*\t(?P<percent>.*)%; (?P<status>.*); (?P<remaining>.*) present").unwrap();
    }

    let output = Command::new("pmset")
        .args(["-g", "batt"])
        .output()
        .expect("failed to execute process pmset");

    let pmset = str::from_utf8(&output.stdout).unwrap();

    let caps = RE.captures(pmset).unwrap();

    PowerMode {
        isOnAC: "AC".eq(&caps["mode"]),
        isOnBattery: "Battery".eq(&caps["mode"]),
        isCharged: "charged".eq(&caps["status"]),
        chargingStatus: String::from(&caps["status"]),
        chargePercent: u16::from_str(&caps["percent"]).unwrap(),
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
