#![allow(dead_code)]

mod api;
mod gateway;
mod structs;

#[macro_use]
extern crate num_derive;

use crate::gateway::auth::identify;
use log::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(debug_assertions)]
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "debug");
        info!("Launching Fern in debug mode, don't get banned!");
    }
    env_logger::init();
    let flr = api::login::login("".into(), "".into()).await?;
    let socket_write = gateway::connect::initiate_gateway_connection().await;
    identify(socket_write.clone(), &flr.token.unwrap()).await;
    // HACK: Remove ts once you get a proper app loop
    let _ = tokio::join!(tokio::spawn(async {
        std::thread::sleep(std::time::Duration::from_secs(3600))
    }));
    Ok(())
}
