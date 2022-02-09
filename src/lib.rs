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
    // connect via identity servcie
    let connection_string = None;
    // alternatively use connection string
    // let connection_string = Some("optional connection string");

    let mut methods = HashMap::<String, DirectMethod>::new();

    methods.insert(
        String::from("closure_no_param_no_result"),
        IotModuleTemplate::make_direct_method(|_in_json| Ok(None)),
    );

    methods.insert(
        String::from("func_params_as_result"),
        Box::new(func_params_as_result),
    );

    let mut template = IotModuleTemplate::new();

    template.run(
        connection_string,
        Some(methods),
        tx_client2app,
        rx_app2client,
    );

    for msg in rx_client2app {
        match msg {
            Message::Desired(state, desired) => {
                if let TwinUpdateState::Partial = state {
                    let mut map: serde_json::Map<String, serde_json::Value> =
                        serde_json::from_value(desired).unwrap();

                    map.remove("$version");

                    tx_app2client
                        .send(Message::Reported(serde_json::Value::Object(map)))
                        .unwrap();
                }
            }
            _ => debug!("Application received unhandled message"),
        }
    }

    template.stop()
}

pub fn func_params_as_result(
    in_json: serde_json::Value,
) -> Result<Option<serde_json::Value>, Box<dyn Error + Send + Sync>> {
    let out_json = json!({
        "called function": "func_params_as_result",
        "your param was": in_json
    });

    Ok(Some(out_json))
}
