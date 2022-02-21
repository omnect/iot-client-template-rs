use azure_iot_sdk::client::*;
use azure_iot_sdk::message::*;
use azure_iot_sdk::twin::Twin;
use log::debug;
use std::collections::HashMap;
use std::error::Error;
use std::sync::{mpsc::Receiver, mpsc::Sender, Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::time;

#[cfg(feature = "systemd")]
use crate::systemd::WatchdogHandler;

#[derive(Debug)]
pub enum Message {
    Desired(TwinUpdateState, serde_json::Value),
    Reported(serde_json::Value),
    Device2Cloud(IotMessage),
    Cloud2Device(IotMessage),
    Authenticated,
    Unauthenticated(UnauthenticatedReason),
    Terminate,
}

struct IotHubClientEventHandler {
    direct_methods: Option<HashMap<String, DirectMethod>>,
    tx: Sender<Message>,
}

impl EventHandler for IotHubClientEventHandler {
    fn handle_connection_status(&self, auth_status: AuthenticationStatus) {
        match auth_status {
            AuthenticationStatus::Authenticated => self.tx.send(Message::Authenticated).unwrap(),
            AuthenticationStatus::Unauthenticated(reason) => {
                self.tx.send(Message::Unauthenticated(reason)).unwrap()
            }
        }
    }

    fn handle_c2d_message(&self, message: IotMessage) -> Result<(), Box<dyn Error + Send + Sync>> {
        self.tx.send(Message::Cloud2Device(message))?;
        Ok(())
    }

    fn get_c2d_message_property_keys(&self) -> Vec<&'static str> {
        vec!["p1", "p2"]
    }

    fn handle_twin_desired(
        &self,
        state: TwinUpdateState,
        desired: serde_json::Value,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        self.tx.send(Message::Desired(state, desired))?;

        Ok(())
    }

    fn get_direct_methods(&self) -> Option<&HashMap<String, DirectMethod>> {
        self.direct_methods.as_ref()
    }
}

pub struct IotClientTemplate {
    thread: Option<JoinHandle<Result<(), Box<dyn Error + Send + Sync + Send + Sync>>>>,
    run: Arc<Mutex<bool>>,
}

impl IotClientTemplate {
    pub fn new() -> Self {
        IotClientTemplate {
            thread: None,
            run: Arc::new(Mutex::new(false)),
        }
    }

    pub fn run<T: Twin>(
        &mut self,
        connection_string: Option<&'static str>,
        direct_methods: Option<HashMap<String, DirectMethod>>,
        tx: Sender<Message>,
        rx: Receiver<Message>,
    ) {
        *self.run.lock().unwrap() = true;

        let running = Arc::clone(&self.run);

        self.thread = Some(thread::spawn(
            move || -> Result<(), Box<dyn Error + Send + Sync + Send + Sync>> {
                let hundred_millis = time::Duration::from_millis(100);
                let event_handler = IotHubClientEventHandler { direct_methods, tx };

                let mut client = match connection_string {
                    Some(cs) => IotHubClient::<T>::from_connection_string(cs, event_handler)?,
                    _ => IotHubClient::from_identity_service(event_handler)?,
                };

                #[cfg(feature = "systemd")]
                let mut wdt = WatchdogHandler::default();

                #[cfg(feature = "systemd")]
                wdt.init()?;

                while *running.lock().unwrap() {
                    match rx.recv_timeout(hundred_millis) {
                        Ok(Message::Reported(reported)) => client.send_reported_state(reported)?,
                        Ok(Message::Device2Cloud(telemetry)) => {
                            client.send_d2c_message(telemetry).map(|_| ())?
                        }
                        Ok(Message::Terminate) => return Ok(()),
                        Ok(_) => debug!("Client received unhandled message"),
                        Err(_) => (),
                    };

                    client.do_work();

                    #[cfg(feature = "systemd")]
                    wdt.notify()?;
                }

                Ok(())
            },
        ));
    }
    pub fn stop(self) -> Result<(), Box<dyn Error + Send + Sync>> {
        *self.run.lock().unwrap() = false;

        self.thread.map_or(Ok(()), |t| t.join().unwrap())
    }

    pub fn make_direct_method<'a, F>(f: F) -> DirectMethod
    where
        F: Fn(serde_json::Value) -> Result<Option<serde_json::Value>, Box<dyn Error + Send + Sync>>
            + 'static
            + Send,
    {
        Box::new(f) as DirectMethod
    }
}
