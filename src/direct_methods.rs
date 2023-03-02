use crate::Message;
use anyhow::Result;
use azure_iot_sdk::client::*;
use serde_json::json;
use std::sync::mpsc::Sender;

pub fn get_direct_methods(tx_app2client: &Sender<Message>) -> Option<DirectMethodMap> {
    let mut methods = DirectMethodMap::new();

    let tx_app2client = tx_app2client.clone();

    methods.insert(
        String::from("closure_send_d2c_message"),
        IotHubClient::make_direct_method(move |_in_json| {
            let msg = IotMessage::builder()
                .set_body(
                    serde_json::to_vec(r#"{"my telemetry message": "hi from device"}"#).unwrap(),
                )
                .set_id("my msg id")
                .set_correlation_id("my correleation id")
                .set_property("my property key", "my property value")
                .set_output_queue("my output queue")
                .build()
                .unwrap();

            tx_app2client
                .send(Message::D2C(msg))
                .unwrap();
            Ok(None)
        }),
    );
    methods.insert(
        String::from("func_echo_params_as_result"),
        Box::new(mirror_func_params_as_result),
    );

    Some(methods)
}

fn mirror_func_params_as_result(in_json: serde_json::Value) -> Result<Option<serde_json::Value>> {
    let out_json = json!({
        "called function": "mirror_func_params_as_result",
        "your param was": in_json
    });

    Ok(Some(out_json))
}
