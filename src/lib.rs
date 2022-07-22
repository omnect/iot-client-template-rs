pub mod client;
pub mod direct_methods;
pub mod message;
#[cfg(feature = "systemd")]
pub mod systemd;
pub mod twin;
use azure_iot_sdk::client::*;
use client::{Client, Message};
use log::{debug, error};
use std::matches;
use std::sync::Once;
use std::sync::{mpsc, Arc, Mutex};

static INIT: Once = Once::new();

#[tokio::main]
pub async fn run() -> Result<(), IotError> {
    let mut client = Client::new();
    let (tx_client2app, rx_client2app) = mpsc::channel();
    let (tx_app2client, rx_app2client) = mpsc::channel();
    let tx_app2client = Arc::new(Mutex::new(tx_app2client));
    let methods = direct_methods::get_direct_methods(Arc::clone(&tx_app2client));

    client.run(None, methods, tx_client2app, rx_app2client);

    for msg in rx_client2app {
        match msg {
            Message::Authenticated => {
                INIT.call_once(|| {
                    #[cfg(feature = "systemd")]
                    systemd::notify_ready();

                    if let Err(e) = twin::report_version(Arc::clone(&tx_app2client)) {
                        error!("Couldn't report version: {}", e);
                    }
                });
            }
            Message::Unauthenticated(reason) => {
                if !matches!(reason, UnauthenticatedReason::ExpiredSasToken) {
                    return Err(IotError::from(format!(
                        "No connection. Reason: {:?}",
                        reason
                    )));
                }
            }
            Message::Desired(state, desired) => {
                twin::update(state, desired, Arc::clone(&tx_app2client));
            }
            Message::C2D(msg) => {
                message::update(msg, Arc::clone(&tx_app2client));
            }
            _ => debug!("Application received unhandled message"),
        }
    }

    client.stop().await
}
