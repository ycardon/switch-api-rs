use sys_info::loadavg;

pub fn get() -> f64 {
    loadavg().unwrap().five
}