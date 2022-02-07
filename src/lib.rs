pub mod iot_module_template;
use azure_iot_sdk::client::*;
use iot_module_template::{IotModuleTemplate, Message};
use log::debug;
use serde_json::json;
use std::collections::HashMap;
use std::error::Error;
use std::sync::mpsc;

pub fn run() -> Result<(), Box<dyn Error + Send + Sync>> {
    let (tx_client2app, rx_client2app) = mpsc::channel();
    let (tx_app2client, rx_app2client) = mpsc::channel();
    let connection_string = Some("add your module connection string here or use eis-utils based provisioning");
    let methods = Some(HashMap::from([
        (
            String::from("closure1"),
            IotModuleTemplate::make_direct_method(|_in_json| Ok(None)),
        ),
        (
            String::from("closure2"),
            IotModuleTemplate::make_direct_method(|in_json| {
                let mut out_json = json!({
                    "closure2": "called",
                    "location": "nowhere"
                });

                out_json["in_json"] = in_json;

                Ok(Some(out_json))
            }),
        ),
        (String::from("func1"), Box::new(func1)),
        (String::from("func2"), Box::new(func2)),
    ]));

    let mut t = IotModuleTemplate::new();

    t.run(connection_string, methods, tx_client2app, rx_app2client);

    for msg in rx_client2app {
        match msg {
            Message::Desired(state, mut desired) => {
                if let TwinUpdateState::Complete = state {
                    desired = desired["desired"].to_owned();
                }

                let mut map: serde_json::Map<String, serde_json::Value> =
                    serde_json::from_value(desired).unwrap();

                map.remove("$version");

                tx_app2client
                    .send(Message::Reported(serde_json::Value::Object(map)))
                    .unwrap();
            }
            _ => debug!("Application received unhandled message"),
        }
    }

    t.stop()
}

pub fn func1(
    _in_json: serde_json::Value,
) -> Result<Option<serde_json::Value>, Box<dyn Error + Send + Sync>> {
    Ok(None)
}

pub fn func2(
    in_json: serde_json::Value,
) -> Result<Option<serde_json::Value>, Box<dyn Error + Send + Sync>> {
    let mut out_json = json!({
        "func2": "called",
        "location": "here"
    });

    out_json["in_json"] = in_json;

    Ok(Some(out_json))
}
