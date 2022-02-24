use crate::Message;
use azure_iot_sdk::client::*;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};

pub fn update(
    state: TwinUpdateState,
    desired: serde_json::Value,
    tx_app2client: Arc<Mutex<Sender<Message>>>,
) {
    if let TwinUpdateState::Partial = state {
        let mut map: serde_json::Map<String, serde_json::Value> =
            serde_json::from_value(desired).unwrap();

        map.remove("$version");

        tx_app2client
            .lock()
            .unwrap()
            .send(Message::Reported(serde_json::Value::Object(map)))
            .unwrap();
    }
}
