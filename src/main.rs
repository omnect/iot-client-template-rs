use serde_json::{json, Value};
use ics_dm_azure_rs::*;


fn twin_callback(data: IotHubModuleEvent, state: IotHubModuleState) {

    println!("twin_callback data: {:?} {:?}", data, state);

    if let IotHubModuleEvent::Twin(new) = data {
        println!("twin_callback new: {}", new);
    }
    let dummy: String = "{\"blubb\": \"supp\"}".to_string();
    self.twin_report(&dummy);
}

fn main() {

    let mut _client = IotHubModuleClient::new(twin_callback);

    // let dummy: String = "{\"blubb\": \"rupp\"}".to_string();
    // _client.twin_report(&dummy);

    println!("Seems everthing is fine!!!");
    _client.do_work();
}
