use crate::Message;
use azure_iot_sdk::client::*;
use serde_json::json;
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

pub fn report_versions(tx_app2client: Arc<Mutex<Sender<Message>>>) -> Result<(), IotError> {
    tx_app2client
        .lock()
        .unwrap()
        .send(Message::Reported(json!({
            "module-version": env!("CARGO_PKG_VERSION"),
            "azure sdk version": IotHubClient::get_sdk_version_string()
        })))?;

    Ok(())
}
