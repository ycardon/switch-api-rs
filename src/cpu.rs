use sys_info::loadavg;

/// Returns CPU load average on 1mn
pub fn get() -> f64 {
    loadavg().unwrap().one
}