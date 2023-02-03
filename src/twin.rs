use crate::Message;
use anyhow::Context;
use anyhow::Result;
use azure_iot_sdk::client::*;
use default_env::default_env;
use network_interface::{NetworkInterface, NetworkInterfaceConfig};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};

pub static NETWORK_NAME_FILTER: &'static str = default_env!("NETWORK_NAME_FILTER", "eth wlan");

pub struct Twin {
    tx: Arc<Mutex<Sender<Message>>>,
}

pub enum ReportProperty {
    Versions,
    NetworkStatus,
}

impl Twin {
    pub fn new(tx: Arc<Mutex<Sender<Message>>>) -> Self {
        Twin { tx }
    }

    pub fn update(&mut self, state: TwinUpdateState, desired: serde_json::Value) -> Result<()> {
        if let TwinUpdateState::Partial = state {
            let mut map: serde_json::Map<String, serde_json::Value> =
                serde_json::from_value(desired).unwrap();

            map.remove("$version");

            self.tx
                .lock()
                .unwrap()
                .send(Message::Reported(serde_json::Value::Object(map)))?;
        }
        
        Ok(())
    }

    pub fn report(&mut self, property: &ReportProperty) -> Result<()> {
        match property {
            ReportProperty::Versions => self.report_versions().context("Couldn't report version"),
            ReportProperty::NetworkStatus => self
                .report_network_status()
                .context("Couldn't report network status"),
        }
    }

    fn report_versions(&mut self) -> Result<()> {
        self.tx.lock().unwrap().send(Message::Reported(json!({
            "module-version": env!("CARGO_PKG_VERSION"),
            "azure-sdk-version": IotHubClient::get_sdk_version_string()
        })))?;

        Ok(())
    }

    fn report_network_status(&mut self) -> Result<()> {
        #[derive(Serialize, Deserialize, Debug)]
        struct NetworkReport {
            name: String,
            addr: String,
            mac: String,
        }

        let reported_interfaces = NetworkInterface::show()?
            .iter()
            .filter(|i| {
                NETWORK_NAME_FILTER
                    .split_whitespace()
                    .any(|f| i.name.starts_with(f))
            })
            .map(|i| NetworkReport {
                name: i.name.clone(),
                addr: i
                    .addr
                    .map_or("none".to_string(), |addr| addr.ip().to_string()),
                mac: i.mac_addr.clone().unwrap_or("none".to_string()),
            })
            .collect::<Vec<NetworkReport>>();

        let t = json!({ "NetworksInterfaces": json!(reported_interfaces) });

        self.tx.lock().unwrap().send(Message::Reported(t))?;

        Ok(())
    }
}
