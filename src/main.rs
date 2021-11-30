use env_logger::{Builder, Env};
use ics_dm_azure::*;
use log::debug;
use std::{thread, time};

fn main() -> Result<(), String> {
    Builder::from_env(Env::default().default_filter_or("info")).init();

    iot_hub_init()?;
    debug!("iot_hub_init successfully");

    let connection = get_connection_info_from_identity_service()?;
    let handle = create_from_connection_string(connection)?;

    set_module_twin_callback(handle)?;
    debug!("set twin callback successfully");

    loop {
        do_work(handle);
        thread::sleep(time::Duration::from_millis(100));
    }
}
