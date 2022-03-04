use crate::{Client, Message};
use azure_iot_sdk::{client::*, message::*, IotError};
use serde_json::json;
use std::collections::HashMap;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};

pub fn get_direct_methods(
    tx_app2client: Arc<Mutex<Sender<Message>>>,
) -> Option<HashMap<String, DirectMethod>> {
    let mut methods: HashMap<String, DirectMethod> = HashMap::new();
    
    methods.insert(
        String::from("closure_send_d2c_message"),
        Client::make_direct_method(move |_in_json| {
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

            tx_app2client.lock().unwrap().send(Message::D2C(msg)).unwrap();
            Ok(None)
        }),
    );
    methods.insert(
        String::from("func_echo_params_as_result"),
        Box::new(func_params_as_result),
    );

    Some(methods)
}

fn func_params_as_result(
    in_json: serde_json::Value,
) -> Result<Option<serde_json::Value>, IotError> {
    let out_json = json!({
        "called function": "func_params_as_result",
        "your param was": in_json
    });

    Ok(Some(out_json))
}
