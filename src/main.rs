use serde_json::{json, Value};
use ics_dm_azure_rs::*;


fn twin_callback(data: IotHubModuleEvent) {

    println!("twin_callback data: {:?}", data);

    if let IotHubModuleEvent::Twin(new) = data {
        println!("twin_callback new: {}", new);
    }

}

fn main() {

    let mut _client = IotHubModuleClient::new(twin_callback);

    println!("Seems everthing is fine!!!");
    _client.do_work();
}
