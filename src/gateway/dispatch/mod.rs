mod ready;

use log::{debug, error};
use ready::{translate_ready, translate_ready_supplemental};

use super::utils::FernWebsocketMessage;

pub async fn handle_dispatch(fwsm: FernWebsocketMessage) {
    let Some(dispatch_event) = fwsm.t.as_ref() else {
        error!("Received Dispatch with no t !");
        return;
    };
    debug!("Received {} dispatch", dispatch_event.as_str());

    match dispatch_event.as_str() {
        "READY" => translate_ready(fwsm),
        "READY_SUPPLEMENTAL" => translate_ready_supplemental(fwsm),
        e => {
            error!("Unimplemented {} {}", e, fwsm.d);
        }
    }
}
