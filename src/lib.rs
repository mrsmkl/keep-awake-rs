#[cfg_attr(target_os = "linux", path = "linux.rs")]
#[cfg_attr(target_os = "windows", path = "windows.rs")]
#[cfg_attr(target_os = "macos", path = "macos.rs")]
mod os;

use crate::os::Holder;

pub fn inhibit(name: &str, reason: &str) -> Result<Holder, Box<dyn std::error::Error>> {
    os::inhibit(name, reason)
}
