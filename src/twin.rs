use crate::Message;
use anyhow::Context;
use anyhow::Result;
use azure_iot_sdk::client::*;
use once_cell::sync::Lazy;
use serde_json::json;
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
}

pub enum ReportProperty {
    Versions,
    MyOtherReportProperty,
}

impl Twin {
    pub fn set_sender(&mut self, tx: Arc<Mutex<Sender<Message>>>) {
        self.tx = Some(tx);
    }

    pub fn update(&mut self, state: TwinUpdateState, desired: serde_json::Value) -> Result<()> {
        match state {
            TwinUpdateState::Partial => {
                self.update_desired(&desired)
            }
            TwinUpdateState::Complete => {
                self.update_desired(&desired["desired"])
            }
        }
    }

    pub fn report(&mut self, property: &ReportProperty) -> Result<()> {
        match property {
            ReportProperty::Versions => self.report_versions().context("Couldn't report version"),
            ReportProperty::MyOtherReportProperty => todo!(),
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

    fn update_desired(&mut self, desired: &serde_json::Value) -> Result<()> {
        let mut map: serde_json::Map<String, serde_json::Value> =
            serde_json::from_value(desired.to_owned()).unwrap();

        map.remove("$version");

        self.tx
            .as_ref()
            .ok_or(anyhow::anyhow!("sender missing").context("report_versions"))?
            .lock()
            .unwrap()
            .send(Message::Reported(serde_json::Value::Object(map)))?;

            Ok(())
    }
}
