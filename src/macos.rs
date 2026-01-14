extern crate libc;
extern crate mach;

use core_foundation::base::TCFType;
use core_foundation::string::{CFString, CFStringRef};

use mach::kern_return::kern_return_t;

pub type IOReturn = kern_return_t;
pub type IOPMAssertionLevel = u32;
pub type IOPMAssertionId = u32;

// const kIOPMAssertionTypeNoDisplaySleep : CFString = CFString::from_static_string("NoDisplaySleepAssertion");
// const kIOPMAssertionTypePreventSystemSleep : CFString = CFString::from_static_string("PreventSystemSleep");

// const K_IOPMASSERTION_LEVEL_OFF : IOPMAssertionLevel = 0;
const K_IOPMASSERTION_LEVEL_ON: IOPMAssertionLevel = 255;

unsafe extern "C" {
    pub fn IOPMAssertionCreateWithName(
        assertion_type: CFStringRef,
        assertion_level: IOPMAssertionLevel,
        assertion_name: CFStringRef,
        assertion_id: &mut IOPMAssertionId,
    ) -> IOReturn;

    pub fn IOPMAssertionRelease(assertion_id: IOPMAssertionId) -> IOReturn;
}

pub struct Holder {
    id: IOPMAssertionId,
}

pub fn inhibit_display(_name: &str, reason: &str) -> Result<Holder, Box<dyn std::error::Error>> {
    inhibit_impl(reason, "NoDisplaySleepAssertion")
}
pub fn inhibit_system(_name: &str, reason: &str) -> Result<Holder, Box<dyn std::error::Error>> {
    inhibit_impl(reason, "PreventSystemSleep")
}

fn inhibit_impl(reason: &str, assertion: &'static str) -> Result<Holder, Box<dyn std::error::Error>> {
    let k_iopmassertion_type_prevent_system_sleep: CFString =
        CFString::from_static_string(assertion);
    let mut id: IOPMAssertionId = 0;
    let reason_cf = CFString::new(reason);
    unsafe {
        let _ret = IOPMAssertionCreateWithName(
            k_iopmassertion_type_prevent_system_sleep.as_concrete_TypeRef(),
            K_IOPMASSERTION_LEVEL_ON,
            reason_cf.as_concrete_TypeRef(),
            &mut id,
        );
    }
    Ok(Holder { id: id })
}

impl Drop for Holder {
    fn drop(&mut self) {
        unsafe {
            let _res = IOPMAssertionRelease(self.id);
        }
    }
}
