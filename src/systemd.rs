use std::error::Error;
use sd_notify::NotifyState;
use std::time::Instant;
use std::sync::Once;

static SD_NOTIFY_ONCE: Once = Once::new();

pub fn notify_ready() {
    SD_NOTIFY_ONCE.call_once(|| {
        let _ = sd_notify::notify(true, &[NotifyState::Ready]);
    });
}

pub struct WatchdogHandler {
    usec: u64,
    now: Option<Instant>,
}

impl Default for WatchdogHandler {
    fn default() -> Self {
        WatchdogHandler {
            usec: u64::MAX,
            now: None,
        }
    }
}

impl WatchdogHandler {
    pub fn init(&mut self) -> Result<(), Box<dyn Error + Send + Sync>> {
        self.usec = u64::MAX;

        if sd_notify::watchdog_enabled(true, &mut self.usec)? {
            self.usec = self.usec / 2;
            self.now = Some(Instant::now());
        }
        
        Ok(())
    }

    pub fn notify(&mut self)  -> Result<(), Box<dyn Error + Send + Sync>> {
        if let Some(ref mut now) =  self.now {
            if u128::from(self.usec) < now.elapsed().as_micros() {
                sd_notify::notify(true, &[NotifyState::Watchdog])?;
                *now = Instant::now();
            }
        }

        Ok(())
    }
}