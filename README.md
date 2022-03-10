# iot-client-template-rs

## What is iot-client-template-rs?

This `iot-client-template-rs` repository provides code to develop Rust based Azure IoT device applications. There are 3 basic approaches to implement Azure compliant device applications:
1. [device twin](https://docs.microsoft.com/en-us/azure/iot-hub/iot-hub-devguide-device-twins): native application representing the device and thus only exists once on a device.
2. [module twin](https://docs.microsoft.com/en-us/azure/iot-hub/iot-hub-devguide-module-twins): native application representing a certain application on the device.
3. [IoTEdge modules](https://docs.microsoft.com/en-us/azure/iot-edge/module-development?view=iotedge-2020-11): **these modules are, not part of this repository**. They are containerized applications running in the IoTEdge runtime. In order to develop C/C++ based IoTEdge modules for ICS_DeviceManagement refer to the [iotedge-module-template](https://github.com/ICS-DeviceManagement/iotedge-module-template) repository.

## Configure build

In your [Cargo.toml](Cargo.toml) file you can configure some common features to be used for `iot-client-template-rs` build.

### Twin type

You have to choose which flavour of iot client you want to build. Thus add one of these twin types to your default features:
1. `device_twin` ([currently not supported with TPM attestation](https://azure.github.io/iot-identity-service/develop-an-agent.html#connecting-your-agent-to-iot-hub))
2. `module_twin` (configured as default)

### systemd integration

The `systemd` feature is an optional feature which is enabled by default. If it is configured `iot-client-template-rs` will:
1. notiy SystemManager when ready (`systemd-notify ready=1`)
2. check if systemd watchdog is enabled (`sd_watchdog_enabled`) and notify SystemManager (`notify watchdog=1`) within the configured watchdog timeout.

## Template and example code

This project shows a basic skeleton for an initial implementation of a Rust based iot device client. It demonstrates how to make use of our Rust [azure-iot-sdk](https://github.com/ICS-DeviceManagement/azure-iot-sdk) in order to connect to Azure iot-hub. Moreover there are examples to show basic communication patterns:

1. **Client**: [client.rs](src/client.rs) implements basic logic needed to communicate with iot-hub. Therefore the `EventHandler` trait is implemented in order to receive new desired properties, direct method calls or cloud to device (C2D) messages from iot-hub. Further the client provides a message channel to send reported properities and device to cloud messages (D2C) to iot-hub.
2. **Device/Module twin**: [message.rs](src/message.rs) implements logic that demonstrates how the client twin can be utilized in applications. As an example desired properties are directly sent back as reported properties.
3. **Direct methods**: [direct_methods.rs](src/direct_methods.rs) implements two functions that serve as direct method and can be synchronously called by iot-hub:
   1. `closure_send_d2c_message`: A closure that doesn't take a parameter and doesn't return a result. The method triggers an outgoing D2C message (@see **3. D2C message**).
   2. `mirror_func_params_as_result`: A function that takes a parameter and returns the same parameter as result.
4. **Device to cloud messages (D2C)**: In [direct_methods.rs](src/direct_methods.rs) there is a direct method call named `closure_send_d2c_message`. It shows how to send a D2C telemetry event to iot-hub.
5. **Cloud to device messages (C2D)**: [message.rs](src/message.rs) demonstrates how the application receives messages sent from cloud. As an example the content of the received message is logged to the console. In order to test that functionality it is the easiest way to configure the application as `device_twin` and send a message from [iot-explorer](https://docs.microsoft.com/en-us/azure/iot-pnp/howto-use-iot-explorer).

All examples can be tested via [iot-explorer](https://docs.microsoft.com/en-us/azure/iot-pnp/howto-use-iot-explorer) or directly via iot-hub view in your Azure portal.

## Platform integration

Both of the supported ICS_DeviceManagement targets - [yocto](https://github.com/ICS-DeviceManagement/meta-ics-dm) and [simulator](https://github.com/ICS-DeviceManagement/simulator) - integrate the `iot-client-template-rs` and serve as an example for device integration.

## Client identity creation in Azure iot-hub

In order to enable the communication between client and cloud a device or module identity needs to be created in Azure iot-hub.

1. **Client identity creation on device via Azure Identity Service (AIS)**: In case your device integrates [AIS](https://azure.github.io/iot-identity-service/), the module creation will be managed automatically on demand. ICS_DeviceManagement yocto layer and simulator support AIS by default.
2. **Manual identity creation and connection string**: As an alternative you might create your device or modules manually in iot-hub and pass the corresponding connection string to the `client.run()` call in [lib.rs](src/lib.rs):

```   
   client.run::<TwinType>(
      TwinType::Module,
      Some("your connection string"),
      Some(methods),
      tx_client2app,
      rx_app2client,
   );  
```
## License

Licensed under either of
* Apache License, Version 2.0, (./LICENSE-APACHE or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license (./LICENSE-MIT or <http://opensource.org/licenses/MIT>)
at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

Copyright (c) 2021 conplement AG