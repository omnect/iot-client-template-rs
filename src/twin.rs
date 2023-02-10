use crate::Message;
use anyhow::Context;
use anyhow::Result;
use azure_iot_sdk::client::*;
use log::{error, info};
use network_interface::{Addr, NetworkInterface, NetworkInterfaceConfig};
use once_cell::sync::Lazy;
use serde::Serialize;
use serde_json::json;
use serde_with::skip_serializing_none;
use std::collections::HashMap;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};

pub static TWIN: Lazy<Mutex<Twin>> = Lazy::new(|| {
    Mutex::new(Twin {
        ..Default::default()
    })
});

#[derive(Default)]
pub struct Twin {
    tx: Option<Arc<Mutex<Sender<Message>>>>,
    include_network_filter: Vec<String>,
}

pub enum ReportProperty {
    Versions,
    NetworkStatus,
}

impl Twin {
    pub fn set_sender(&mut self, tx: Arc<Mutex<Sender<Message>>>) {
        self.tx = Some(tx);
    }

    pub fn update(&mut self, state: TwinUpdateState, desired: serde_json::Value) -> Result<()> {
        match state {
            TwinUpdateState::Partial => {
                self.update_include_network_filter(desired["include_network_filter"].as_array())
            }
            TwinUpdateState::Complete => self.update_include_network_filter(
                desired["desired"]["include_network_filter"].as_array(),
            ),
        }
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
        self.tx
            .as_ref()
            .ok_or(anyhow::anyhow!("sender missing").context("report_versions"))?
            .lock()
            .unwrap()
            .send(Message::Reported(json!({
                "module-version": env!("CARGO_PKG_VERSION"),
                "azure-sdk-version": IotHubClient::get_sdk_version_string()
            })))?;

        Ok(())
    }

    fn update_include_network_filter(
        &mut self,
        include_network_filter: Option<&Vec<serde_json::Value>>,
    ) -> Result<()> {
        let mut new_include_network_filter = if include_network_filter.is_some() {
            include_network_filter
                .unwrap()
                .iter()
                .filter(|e| {
                    if !e.is_string() {
                        error!(
                            "unexpected format in desired include_network_filter. ignore: {}",
                            e.to_string()
                        );
                    }
                    e.is_string()
                })
                .map(|e| e.as_str().unwrap().to_string().to_lowercase())
                .collect()
        } else {
            vec!["*".to_string()]
        };

        // enforce entries only exists once
        new_include_network_filter.sort();
        new_include_network_filter.dedup();

        // check if desired include_network_filter changed
        if self.include_network_filter.ne(&new_include_network_filter) {
            self.include_network_filter = new_include_network_filter;
            self.report_network_status()
        } else {
            info!("desired include_network_filter didn't change");
            Ok(())
        }
    }

    fn report_network_status(&mut self) -> Result<()> {
        #[skip_serializing_none]
        #[derive(Serialize)]
        struct NetworkReport {
            #[serde(default)]
            name: String,
            mac: String,
            addr_v4: Option<Vec<String>>,
            addr_v6: Option<Vec<String>>,
        }

        let mut interfaces: HashMap<String, NetworkReport> = HashMap::new();

        NetworkInterface::show()
            .context("report_network_status")?
            .iter()
            .filter(|i| {
                self.include_network_filter.iter().any(|f| {
                    let name = i.name.to_lowercase();
                    match (f.starts_with("*"), f.ends_with("*"), f.len()) {
                        (_, _, 0) => false,                                     // ""
                        (a, b, 1) if a || b => true,                            // "*"
                        (true, true, len) => name.contains(&f[1..len - 1]),     // ""*...*"
                        (true, false, len) => name.ends_with(&f[1..len]),       // "*..."
                        (false, true, len) => name.starts_with(&f[0..len - 1]), // "...*"
                        _ => name.eq(f),                                        // "..."
                    }
                })
            })
            .for_each(|i| {
                let entry = interfaces.entry(i.name.clone()).or_insert(NetworkReport {
                    addr_v4: None,
                    addr_v6: None,
                    mac: i.mac_addr.clone().unwrap_or_else(|| "none".to_string()),
                    name: i.name.clone(),
                });

                match i.addr {
                    Some(Addr::V4(addr)) => entry
                        .addr_v4
                        .get_or_insert(vec![])
                        .push(addr.ip.to_string()),
                    Some(Addr::V6(addr)) => entry
                        .addr_v6
                        .get_or_insert(vec![])
                        .push(addr.ip.to_string()),
                    None => error!("report_network_status: ip address is missing"),
                };
            });

        self.tx
            .as_ref()
            .ok_or(anyhow::anyhow!("sender missing").context("report_network_status"))?
            .lock()
            .unwrap()
            .send(Message::Reported(json!({
                "network_interfaces":
                    json!(interfaces.into_values().collect::<Vec<NetworkReport>>())
            })))
            .context("report_network_status")?;

        Ok(())
    }
}
