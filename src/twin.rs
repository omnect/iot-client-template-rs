use crate::Message;
use anyhow::{Context, Result};
use azure_iot_sdk::client::*;
use log::info;
use once_cell::sync::OnceCell;
use serde_json::json;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex, MutexGuard};

static INSTANCE: OnceCell<Mutex<Twin>> = OnceCell::new();

pub struct TwinInstance {
    inner: &'static Mutex<Twin>,
}

pub fn get_or_init(tx: Option<Arc<Mutex<Sender<Message>>>>) -> TwinInstance {
    if tx.is_some() {
        TwinInstance {
            inner: INSTANCE.get_or_init(|| Mutex::new(Twin { tx })),
        }
    } else {
        TwinInstance {
            inner: INSTANCE.get().unwrap(),
        }
    }
}

struct TwinLock<'a> {
    inner: MutexGuard<'a, Twin>,
}

impl TwinInstance {
    fn lock(&self) -> TwinLock<'_> {
        TwinLock {
            inner: self.inner.lock().unwrap_or_else(|e| e.into_inner()),
        }
    }

    pub fn report(&self, property: &ReportProperty) -> Result<()> {
        self.lock().inner.report(property)
    }

    pub fn update(&self, state: TwinUpdateState, desired: serde_json::Value) -> Result<()> {
        self.lock().inner.update(state, desired)
    }
}

#[derive(Default)]
struct Twin {
    tx: Option<Arc<Mutex<Sender<Message>>>>,
}

pub enum ReportProperty {
    Versions,
    MyOtherReportProperty,
}

impl Twin {
    fn update(&mut self, state: TwinUpdateState, desired: serde_json::Value) -> Result<()> {
        match state {
            TwinUpdateState::Partial => self.update_desired(&desired),
            TwinUpdateState::Complete => self.update_desired(&desired["desired"]),
        }
    }

    fn report(&mut self, property: &ReportProperty) -> Result<()> {
        match property {
            ReportProperty::Versions => self.report_versions().context("Couldn't report version"),
            ReportProperty::MyOtherReportProperty => unimplemented!(),
        }
    }

    fn report_versions(&mut self) -> Result<()> {
        self.report_impl(json!({
            "module-version": env!("CARGO_PKG_VERSION"),
            "azure-sdk-version": IotHubClient::get_sdk_version_string()
        }))
        .context("report_versions")
    }

    fn update_desired(&mut self, desired: &serde_json::Value) -> Result<()> {
        let mut map: serde_json::Map<String, serde_json::Value> =
            serde_json::from_value(desired.to_owned()).unwrap();

        map.remove("$version");

        self.report_impl(serde_json::Value::Object(map))
            .context("update_desired")
    }

    fn report_impl(&mut self, value: serde_json::Value) -> Result<()> {
        info!("report: \n{:?}", value);

        self.tx
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("tx channel missing"))?
            .lock()
            .unwrap()
            .send(Message::Reported(value))
            .map_err(|err| err.into())
    }
}
