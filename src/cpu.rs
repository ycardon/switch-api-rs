use sys_info::loadavg;

/// Returns CPU load average on 5mn
pub fn get() -> f64 {
    loadavg().unwrap().five
}