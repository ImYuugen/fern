#![allow(dead_code)]

#[macro_use]
extern crate num_derive;

use log::info;

mod api;
mod websocket;

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "debug");
        info!("Launching Fern in debug mode, don't get banned!");
    }
    env_logger::init();
    // let _flr = api::login::login("".into(), "".into());
    let _socket_write = websocket::connect::initiate_websocket_con().await;
}
