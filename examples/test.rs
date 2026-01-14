use keep_awake::*;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Here!");
    let _obj = inhibit_display("Plumo Snark steup", "very important")?;
    // drop(obj);
    std::thread::sleep(Duration::from_millis(2000 * 1000));
    Ok(())
}
