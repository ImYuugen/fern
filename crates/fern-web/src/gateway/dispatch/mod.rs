mod messages;
mod ready;

use log::{debug, error};
use messages::{message_create, message_delete, message_update};
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
        "MESSAGE_CREATE" => message_create(fwsm),
        "MESSAGE_DELETE" => message_delete(fwsm),
        "MESSAGE_UPDATE" => message_update(fwsm),
        e => {
            error!("Unimplemented {} {}", e, fwsm.d);
        }
    }
}
