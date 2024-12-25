#![allow(dead_code)]
#![allow(unused_variables)]

use log::info;

mod api;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(debug_assertions)]
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "trace");
        info!("Launching Fern in debug mode, don't get banned!");
    }
    env_logger::init();
    Ok(())
}
