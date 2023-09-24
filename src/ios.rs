
pub struct Holder {  }

use objc::*;
use objc::runtime::{ Object, YES, NO };

pub fn inhibit_system(_name: &str, _reason: &str) -> Result<Holder, Box<dyn std::error::Error>> {
    Err("not implemented on iOS".into())
}

pub fn inhibit_display(_name: &str, _reason: &str) -> Result<Holder, Box<dyn std::error::Error>> {
    unsafe {
        let app: *const Object = msg_send![class!(UIApplication), sharedApplication];
        let () = msg_send![app, setIdleTimerDisabled: YES];
    }

    Ok(Holder { })
}

impl Drop for Holder {
    fn drop(&mut self) {
        unsafe {
            let app: *const Object = msg_send![class!(UIApplication), sharedApplication];
            let () = msg_send![app, setIdleTimerDisabled: NO];
        }
    }
}
