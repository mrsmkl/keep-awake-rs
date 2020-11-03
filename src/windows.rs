

use winapi::um::winnt::{EXECUTION_STATE, ES_SYSTEM_REQUIRED, ES_DISPLAY_REQUIRED, ES_CONTINUOUS};
use winapi::um::winbase::{SetThreadExecutionState};

pub struct Holder {
}

pub fn inhibit(_name: &str, _reason: &str) -> Result<Holder, Box<dyn std::error::Error>> {
    unsafe {
        let state = ES_DISPLAY_REQUIRED | ES_CONTINUOUS;
        let res = SetThreadExecutionState(state);
        println!("did somthing {:?} {:?}", state, res);
    }
    Ok(Holder{})
}
