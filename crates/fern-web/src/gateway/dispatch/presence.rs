use log::{debug, info, warn};

use crate::{gateway::utils::FernWebsocketMessage, structs::user::Activity};

pub fn presence_update(fwsm: FernWebsocketMessage) {
    warn!("Need to miplemetpl {:?}", fwsm.d);
}

#[derive(serde::Deserialize, Debug)]
struct SessionsReplaceEvent {
    client_info: ClientInfo,
    activities: Vec<Activity>,
}
#[derive(serde::Deserialize, Debug)]
struct ClientInfo {
    desktop: String,
    os: String,
    version: u8,
}

pub fn sessions_replace(fwsm: FernWebsocketMessage) {
    let Some(presence) = fwsm.d.get(0) else {
        warn!("Empty SESSIONS_REPLACE, concerning !");
        return;
    };
    debug!("{:?}", fwsm.d);
    info!(
        "Own presence updated as {} on {} with {} activities",
        presence.get("status").unwrap(),
        presence.get("client_info").unwrap(),
        presence.get("activities").unwrap(),
    );
}
