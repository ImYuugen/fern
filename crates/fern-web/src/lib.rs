#![allow(dead_code)]

use gateway::auth::identify;
use log::info;

mod api;
mod gateway;
mod structs;

#[macro_use]
extern crate num_derive;

pub async fn start_loop() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: Implement actions
    info!("Yo yo I just started tha loop-di-loop");
    let flr = api::login::login("".into(), "".into()).await?;
    info!("Yo yo I logged in it's bussin'");
    let socket_write = gateway::connect::initiate_gateway_connection().await;
    info!("Ong we opened gateway");
    identify(socket_write.clone(), &flr.token.unwrap()).await;
    info!("Ayoooo I identified ong");
    tokio::signal::ctrl_c().await?;
    gateway::disconnect::disconnect(socket_write).await;
    info!("Disconnecting lowk coward behaviour fr ong");
    Ok(())
}
