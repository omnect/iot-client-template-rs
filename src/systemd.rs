use anyhow::Result;
use log::{trace, info};
use sd_notify::NotifyState;
use std::sync::Once;
use std::time::Instant;

static SD_NOTIFY_ONCE: Once = Once::new();

pub fn notify_ready() {
    SD_NOTIFY_ONCE.call_once(|| {
        info!("notify ready=1");
        let _ = sd_notify::notify(false, &[NotifyState::Ready]);
    });
}

#[derive(Default)]
pub struct WatchdogHandler {
    usec: u64,
    now: Option<Instant>,
}

impl WatchdogHandler {
    pub fn new() -> Self {
        let mut usec = u64::MAX;
        let mut now = None;

        if sd_notify::watchdog_enabled(false, &mut usec) {
            usec /= 2;
            now = Some(Instant::now());
        }

        info!(
            "watchdog settings: enabled: {} interval: {}µs",
            now.is_some(),
            usec
        );

        WatchdogHandler { usec, now }
    }

    pub fn notify(&mut self) -> Result<()> {
        if let Some(ref mut now) = self.now {
            if u128::from(self.usec) < now.elapsed().as_micros() {
                trace!("notify watchdog=1");
                sd_notify::notify(false, &[NotifyState::Watchdog])?;
                *now = Instant::now();
            }
        }

        Ok(())
    }
}
