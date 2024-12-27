use log::{debug, trace};

use super::utils::*;

pub async fn heartbeat_loop(fswm: FernWebsocketMessage) {
    let hearbeat = fswm
        .d
        .get("heartbeat_interval")
        .expect("No heartbeat_interval !?")
        .as_u64()
        .expect("heartbeat_interval was not a number !?");
    debug!("Starting heartbeat with interval of {}ms", hearbeat);
    let mut interval = tokio::time::interval(std::time::Duration::from_millis(hearbeat));
    loop {
        interval.tick().await;
        // TODO: Send hearbeat
        trace!("*Thump* sent heatbeat");
    }
}
