use regex::Regex;
use serde::Serialize;

/// Parses `pmset -g batt` output and returns the current MacBook power mode
pub fn get() -> PowerMode {
    lazy_static! {
        static ref RE: Regex = Regex::new(r".*'(?P<mode>.*) Power'\n.*\t(?P<percent>.*)%; (?P<status>.*); (?P<remaining>.*) present/mg").unwrap();
    }

    let text = "Now drawing from 'Battery Power'
-InternalBattery-0 (id=21233763)	80%; discharging; 5:49 remaining present: true";

    let caps = RE.captures(text).unwrap();

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
