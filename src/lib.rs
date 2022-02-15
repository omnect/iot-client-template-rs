pub mod iot_module_template;
use azure_iot_sdk::client::*;
use azure_iot_sdk::message::*;
use iot_module_template::{IotModuleTemplate, Message};
use log::debug;
use serde_json::json;
use std::collections::HashMap;
use std::error::Error;
use std::sync::{mpsc, Arc, Mutex};

pub fn run() -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut template = IotModuleTemplate::new();
    let (tx_client2app, rx_client2app) = mpsc::channel();
    let (tx_app2client, rx_app2client) = mpsc::channel();

    let tx_app2client = Arc::new(Mutex::new(tx_app2client));

    // connect via identity servcie
    let connection_string = None;
    // alternatively use connection string
    //let connection_string = Some("optional connection string");

    let mut methods = HashMap::<String, DirectMethod>::new();

    let tx_closure = Arc::clone(&tx_app2client);

    methods.insert(
        String::from("closure_send_d2c_message"),
        IotModuleTemplate::make_direct_method(move |_in_json| {
            let msg = IotMessage::builder()
                .set_body(
                    serde_json::to_vec("{ \"my telemetry message\": \"hi from device\" }").unwrap(),
                )
                .set_id(String::from("my msg id"))
                .set_correlation_id(String::from("my correleation id"))
                .set_property(
                    String::from("my property key"),
                    String::from("my property value"),
                )
                .set_output_queue(String::from("my output queue"))
                .build();

            tx_closure
                .lock()
                .unwrap()
                .send(Message::Telemetry(msg))
                .unwrap();
            Ok(None)
        }),
    );

    methods.insert(
        String::from("func_echo_params_as_result"),
        Box::new(func_params_as_result),
    );

    template.run(
        connection_string,
        Some(methods),
        tx_client2app,
        rx_app2client,
    );

    for msg in rx_client2app {
        match msg {
            Message::Unauthenticated(reason) => {
                template.stop().unwrap();
                return Err(Box::<dyn Error + Send + Sync>::from(format!(
                    "No connection. Reason: {:?}",
                    reason
                )));
            }
            Message::Desired(state, desired) => {
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
