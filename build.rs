use std::env;

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS");
    match target_os.as_ref().map(|x| &**x) {
        Ok("macos") => println!("cargo:rustc-link-lib=framework=IOKit"),
        _ => {}
    }
}
