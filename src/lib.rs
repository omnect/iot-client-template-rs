#[cfg(not(any(feature = "device_twin", feature = "module_twin")))]
compile_error!("Either feature \"device_twin\" or \"module_twin\" must be enabled for this crate.");

#[cfg(all(feature = "device_twin", feature = "module_twin"))]
compile_error!("Either feature \"device_twin\" or \"module_twin\" must be enabled for this crate.");

#[cfg(feature = "device_twin")]
type TwinType = DeviceTwin;

#[cfg(feature = "module_twin")]
type TwinType = ModuleTwin;

pub mod iot_client_template;
#[cfg(feature = "systemd")]
pub mod systemd;
use azure_iot_sdk::client::*;
use azure_iot_sdk::message::*;
use azure_iot_sdk::twin::*;
use iot_client_template::{IotClientTemplate, Message};
use log::debug;
use serde_json::json;
use std::collections::HashMap;
use std::error::Error;
use std::sync::{mpsc, Arc, Mutex};

pub fn run() -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut template = IotClientTemplate::new();
    let (tx_client2app, rx_client2app) = mpsc::channel();
    let (tx_app2client, rx_app2client) = mpsc::channel();
    let tx_app2client = Arc::new(Mutex::new(tx_app2client));
    let mut methods = HashMap::<String, DirectMethod>::new();

    let tx_closure = Arc::clone(&tx_app2client);

    methods.insert(
        String::from("closure_send_d2c_message"),
        IotClientTemplate::make_direct_method(move |_in_json| {
            let msg = IotMessage::builder()
                .set_body(
                    serde_json::to_vec(r#"{"my telemetry message": "hi from device"}"#).unwrap(),
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
                .send(Message::Device2Cloud(msg))
                .unwrap();
            Ok(None)
        }),
    );

    methods.insert(
        String::from("func_echo_params_as_result"),
        Box::new(func_params_as_result),
    );

    template.run::<TwinType>(None, Some(methods), tx_client2app, rx_app2client);

    for msg in rx_client2app {
        match msg {
            Message::Authenticated => {
                #[cfg(feature = "systemd")]
                systemd::notify_ready();
            }
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
            Message::Cloud2Device(msg) => {
                debug!(
                    "Received c2d message with \n body: {:?}\n properties: {:?} \n system properties: {:?}",
                    std::str::from_utf8(&msg.body).unwrap(), msg.properties, msg.system_properties
                );
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
