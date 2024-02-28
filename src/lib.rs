#[cfg_attr(target_os = "linux", path = "linux.rs")]
#[cfg_attr(target_os = "windows", path = "windows.rs")]
#[cfg_attr(target_os = "macos", path = "macos.rs")]
#[cfg_attr(target_os = "android", path = "android.rs")]
#[cfg_attr(target_os = "ios", path = "ios.rs")]
mod os;

// Good reference for other platforms: https://github.com/chromium/chromium/tree/main/services/device/wake_lock/power_save_blocker

pub use os::*;
