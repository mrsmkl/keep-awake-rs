use windows::Win32::System::Power::{
    SetThreadExecutionState, ES_CONTINUOUS, ES_DISPLAY_REQUIRED, ES_SYSTEM_REQUIRED, EXECUTION_STATE
};

pub struct Holder {}

pub fn inhibit_system(_name: &str, _reason: &str) -> Result<Holder, Box<dyn std::error::Error>> {
    unsafe {
        let state = ES_SYSTEM_REQUIRED | ES_CONTINUOUS;
        let _res = SetThreadExecutionState(state);
    }
    Ok(Holder {})
}

pub fn inhibit_display(_name: &str, _reason: &str) -> Result<Holder, Box<dyn std::error::Error>> {
    unsafe {
        let state = ES_DISPLAY_REQUIRED | ES_CONTINUOUS;
        let _res = SetThreadExecutionState(state);
    }
    Ok(Holder {})
}

impl Drop for Holder {
    fn drop(&mut self) {
        unsafe {
            let _res = SetThreadExecutionState(EXECUTION_STATE(0));
        }
    }
}
