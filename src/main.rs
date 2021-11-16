use env_logger::{Builder, Env};
use ics_dm_azure_rs::*;
use log::debug;
use std::{thread, time};

fn main() {
    Builder::from_env(Env::default().default_filter_or("info")).init();

    let result = iot_hub_init();
    if result != 0 {
        panic!("iot_hub_init not OK!");
    }

    let handle;
    let connection;
    match get_connection_info_from_identity_service() {
        Ok(date) => {
            connection = date;
            handle = create_from_connection_string(connection);
        }
        Err(e) => {
            panic!("{}", e);
        }
    }

    if handle.is_null() {
        panic!("no valid handle received");
    }

    match set_module_twin_callback(handle) {
        Ok(()) => {
            debug!("set twin callback successfully");
        }
        Err(e) => {
            panic!("{}", e);
        }
    }

    loop {
        do_work(handle);
        let hundred_millis = time::Duration::from_millis(100);
        thread::sleep(hundred_millis);
    }
}
