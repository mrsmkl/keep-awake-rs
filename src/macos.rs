
extern crate CoreFoundation_sys as cf;
extern crate libc;
extern crate mach;

use cf::{CFTypeRef,CFDictionaryRef,CFMutableDictionaryRef,CFStringRef,CFAllocatorRef};
use libc::{c_void,c_char,c_int,size_t,uintptr_t,uint32_t};

use mach::kern_return::{kern_return_t,KERN_SUCCESS};

pub type IOReturn = kern_return_t;
pub type IOPMAssertionLevel = uint32_t;
pub type IOPMAssertionId = uint32_t;

extern "C" {
    pub fn IOPMAssertionCreateWithName(
        assertion_type: CFStringRef,
        assertion_level: IOPMAssertionLevel,
        assertion_name: CFStringRef,
        assertion_id: &mut IOPMAssertionId) -> IOReturn;
    
    pub fn IOPMAssertionRelease(assertion_id: IOPMAssertionId) -> IOReturn;
}

pub struct Holder {
}

pub fn inhibit(name: &str, reason: &str) -> Result<Holder, Box<dyn std::error::Error>> {
    Ok(Holder {  })
}
