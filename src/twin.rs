use super::systemd;
use crate::systemd::WatchdogHandler;
use anyhow::{anyhow, Context, Result};
use azure_iot_sdk::client::*;
use futures_util::FutureExt;
use log::{error, info};
use serde_json::json;
use tokio::{
    select,
    sync::mpsc,
    time::{interval, timeout, Duration},
};

pub struct Twin {
    iothub_client: Box<dyn IotHub>,
    authenticated_once: bool,
    tx_reported_properties: mpsc::Sender<serde_json::Value>,
    tx_outgoing_message: mpsc::Sender<IotMessage>,
}

impl Twin {
    pub fn new(
        client: Box<dyn IotHub>,
        tx_reported_properties: mpsc::Sender<serde_json::Value>,
        tx_outgoing_message: mpsc::Sender<IotMessage>,
    ) -> Self {
        Twin {
            iothub_client: client,
            tx_reported_properties: tx_reported_properties.clone(),
            tx_outgoing_message: tx_outgoing_message.clone(),
            authenticated_once: false,
        }
    }

    async fn handle_connection_status(&mut self, auth_status: AuthenticationStatus) -> Result<()> {
        info!("auth_status: {auth_status:#?}");

        match auth_status {
            AuthenticationStatus::Authenticated => {
                if !self.authenticated_once {
                    systemd::notify_ready();

                    self.authenticated_once = true;
                };
            }
            AuthenticationStatus::Unauthenticated(reason) => {
                anyhow::ensure!(
                    matches!(reason, UnauthenticatedReason::ExpiredSasToken),
                    "No connection. Reason: {reason:?}"
                );
            }
        }

        Ok(())
    }

    async fn handle_desired(
        &mut self,
        state: TwinUpdateState,
        desired: serde_json::Value,
    ) -> Result<()> {
        info!("desired: {state:#?}, {desired}");

        let desired = match state {
            TwinUpdateState::Partial => &desired,
            TwinUpdateState::Complete => &desired["desired"],
        };

        let mut map: serde_json::Map<String, serde_json::Value> =
            serde_json::from_value(desired.to_owned()).unwrap();

        map.remove("$version");

        self.tx_reported_properties
            .send(serde_json::Value::Object(map))
            .await
            .map_err(|err| err.into())
    }

    async fn handle_direct_method(
        &mut self,
        method_name: String,
        payload: serde_json::Value,
    ) -> Result<Option<serde_json::Value>> {
        info!("handle_direct_method: {method_name} with payload: {payload}");

        match method_name.as_str() {
            "closure_send_d2c_message" => {
                let message = IotMessage::builder()
                    .set_body(
                        serde_json::to_vec(r#"{"my telemetry message": "hi from device"}"#)
                            .unwrap(),
                    )
                    .set_id("my msg id")
                    .set_correlation_id("my correleation id")
                    .set_property("my property key", "my property value")
                    .set_output_queue("my output queue")
                    .build()
                    .unwrap();

                self.tx_outgoing_message.send(message).await?;
                Ok(None)
            }
            "func_echo_params_as_result" => {
                let out_json = json!({
                    "called function": "mirror_func_params_as_result",
                    "your param was": payload
                });

                Ok(Some(out_json))
            }
            _ => Err(anyhow!("direct method unknown")),
        }
    }

    async fn handle_report_property(&mut self, properties: serde_json::Value) -> Result<()> {
        info!("report: {properties}");

        match timeout(
            Duration::from_secs(5),
            self.iothub_client.twin_report(properties),
        )
        .await
        {
            Ok(result) => result.context("handle_report_property: couldn't report property"),
            Err(_) => Err(anyhow!("handle_report_property: timeout occured")),
        }
    }

    async fn handle_incoming_message(&mut self, message: IotMessage) -> Result<DispositionResult> {
        info!(
            "received C2D message with \n body: {:?}\n properties: {:?} \n system properties: {:?}",
            std::str::from_utf8(&message.body).unwrap(),
            message.properties,
            message.system_properties
        );
        Ok(DispositionResult::Accepted)
    }

    async fn handle_outgoing_message(&mut self, message: IotMessage) -> Result<()> {
        info!("send message: {message:#?}");

        match timeout(
            Duration::from_secs(5),
            self.iothub_client.send_d2c_message(message),
        )
        .await
        {
            Ok(result) => result.context("handle_outgoing_message: couldn't send message"),
            Err(_) => Err(anyhow!("handle_outgoing_message: timeout occurred")),
        }
    }

    pub async fn run(connection_string: Option<&str>) -> Result<()> {
        let (tx_connection_status, mut rx_connection_status) = mpsc::channel(100);
        let (tx_twin_desired, mut rx_twin_desired) = mpsc::channel(100);
        let (tx_direct_method, mut rx_direct_method) = mpsc::channel(100);
        let (tx_reported_properties, mut rx_reported_properties) = mpsc::channel(100);
        let (tx_incoming_message, mut rx_incoming_message) = mpsc::channel(100);
        let (tx_outgoing_message, mut rx_outgoing_message) = mpsc::channel(100);
        let mut sd_notify_interval = interval(Duration::from_secs(10));
        let mut wdt = WatchdogHandler::new();
        let incoming_message_observer =
            IncomingMessageObserver::new(tx_incoming_message.clone(), vec![]);

        let client = match IotHubClient::client_type() {
            _ if connection_string.is_some() => IotHubClient::from_connection_string(
                connection_string.unwrap(),
                Some(tx_connection_status.clone()),
                Some(tx_twin_desired.clone()),
                Some(tx_direct_method.clone()),
                Some(incoming_message_observer),
            )?,
            ClientType::Device | ClientType::Module => {
                IotHubClient::from_identity_service(
                    Some(tx_connection_status.clone()),
                    Some(tx_twin_desired.clone()),
                    Some(tx_direct_method.clone()),
                    None,
                )
                .await?
            }
            ClientType::Edge => IotHubClient::from_edge_environment(
                Some(tx_connection_status.clone()),
                Some(tx_twin_desired.clone()),
                Some(tx_direct_method.clone()),
                None,
            )?,
        };

        let mut twin = Self::new(client, tx_reported_properties, tx_outgoing_message);

        loop {
            select! (
                _ = sd_notify_interval.tick() => {
                    wdt.notify()?;
                }
                status = rx_connection_status.recv() => {
                    twin.handle_connection_status(status.unwrap()).await?;
                },
                desired = rx_twin_desired.recv() => {
                    let (state, desired) = desired.unwrap();
                    twin.handle_desired(state, desired).await.unwrap_or_else(|e| error!("twin update desired properties: {e:#?}"));
                },
                reported = rx_reported_properties.recv() => {
                    twin.handle_report_property(reported.unwrap()).await?;
                },
                incoming_message = rx_incoming_message.recv() => {
                    let (message, tx_result) = incoming_message.unwrap();
                    let fut = twin.handle_incoming_message(message);
                    tokio::pin!(fut);
                    if let Some(result) = fut.as_mut().now_or_never() {
                        if tx_result.send(result).is_err() {
                            error!("run: receiver dropped");
                        }
                    } else {
                        if tx_result.send(Ok(DispositionResult::Accepted)).is_err() {
                            error!("run: receiver dropped");
                        }
                        if let Err(e) = fut.await {
                            error!("run: handle_direct_method: {e}");
                        }
                    };
                },
                outgoing_message = rx_outgoing_message.recv() => {
                    twin.handle_outgoing_message(outgoing_message.unwrap()).await?;
                },
                direct_methods = rx_direct_method.recv() => {
                    /*
                        azure-iot-sdk-c calls direct method handler blocking in order to wait for the method result.
                        Since sdk uses only a single thread to handle all callbacks a deadlock might be the result,
                        if the method itself e.g. calls twin_report() which also blocks until the confirmation via
                        callback is received.
                        In order to workaround this issue we call now_or_never() which either returns Some(result) in case
                        future is ready or None otherwise. In the second case we assume the direct method succeeded and
                        has no result so we return Ok(None).
                    */
                    let (name, payload, tx_result) = direct_methods.unwrap();
                    let fut = twin.handle_direct_method(name, payload);
                    tokio::pin!(fut);
                    if let Some(result) = fut.as_mut().now_or_never() {
                        if tx_result.send(result).is_err() {
                            error!("run: receiver dropped");
                        }
                    } else {
                        if tx_result.send(Ok(None)).is_err() {
                            error!("run: receiver dropped");
                        }
                        if let Err(e) = fut.await {
                            error!("run: handle_direct_method: {e}");
                        }
                    };
                }
            );
        }
    }
}
