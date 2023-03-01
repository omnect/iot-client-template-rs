use crate::Message;
use azure_iot_sdk::client::IotMessage;
use log::debug;
use std::sync::mpsc::Sender;

pub fn update(msg: IotMessage, _tx_app2client: &Sender<Message>) {
    debug!(
        "Received C2D message with \n body: {:?}\n properties: {:?} \n system properties: {:?}",
        std::str::from_utf8(&msg.body).unwrap(),
        msg.properties,
        msg.system_properties
    );
}
