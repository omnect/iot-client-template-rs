# What is iot-client-template-rs?

This `iot-client-template-rs` repository provides code to develop Rust based Azure IoT device applications. There are 3 basic approaches to implement Azure compliant device applications:
1. [device twin](https://docs.microsoft.com/en-us/azure/iot-hub/iot-hub-devguide-device-twins): native application representing the device and thus only exists once on a device.
2. [module twin](https://docs.microsoft.com/en-us/azure/iot-hub/iot-hub-devguide-module-twins): native application representing a certain application on the device.
3. [IoTEdge modules](https://docs.microsoft.com/en-us/azure/iot-edge): containerized applications running on device IoTEdge runtime.

# Build

## Library dependencies

Please refer to [azure-iot-sdk-sys](https://github.com/omnect/azure-iot-sdk-sys/blob/main/README.md) documentation in order to provide mandatory libraries needed to build `iot-client-template-rs` successfully.

An error output similar to the following example indicates that libraries are not set correctly:
```
--- stderr
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: `"pkg-config" "--libs" "--cflags" "azure-iotedge-sdk-dev"` did not exit successfully: exit status: 1
error: could not find system library 'azure-iotedge-sdk-dev' required by the 'azure-iot-sdk-sys' crate

--- stderr
Package azure-iotedge-sdk-dev was not found in the pkg-config search path.
Perhaps you should add the directory containing `azure-iotedge-sdk-dev.pc'
to the PKG_CONFIG_PATH environment variable
No package 'azure-iotedge-sdk-dev' found
```

## Configure build options

First of all you might have to configure [client type](#client-type) and [systemd](#systemd-integration) usage in [Cargo.toml](Cargo.toml) file to be used for `iot-client-template-rs` build.

### Client type

You have to choose which flavour of iot client you want to build. Thus set one of these client types in your default features:
1. `device_client` ([currently not supported with TPM attestation](https://azure.github.io/iot-identity-service/develop-an-agent.html#connecting-your-agent-to-iot-hub))
2. `module_client` (default)
3. `edge_client`

### systemd integration

The `systemd` feature is an optional feature which is enabled by default. If it is configured `iot-client-template-rs` will:
1. notify SystemManager when ready (`systemd-notify ready=1`)
2. check if systemd watchdog is enabled (`sd_watchdog_enabled`) and notify SystemManager (`notify watchdog=1`) within the configured watchdog timeout.

# Template and example code

This project shows a basic skeleton for an initial implementation of a Rust based iot device client. It demonstrates how to make use of our Rust [azure-iot-sdk](https://github.com/omnect/azure-iot-sdk) in order to connect to Azure iot-hub. Moreover there are examples to show basic communication patterns:

1. **Client**: [client.rs](src/client.rs) implements basic logic needed to communicate with iot-hub. Therefore the `EventHandler` trait is implemented in order to receive new desired properties, direct method calls or cloud to device (C2D) messages from iot-hub. Further the client provides a message channel to send reported properities and device to cloud messages (D2C) to iot-hub.
2. **Twin properties**: [message.rs](src/message.rs) implements logic that demonstrates how the client twin can be utilized in applications. As an example desired properties are directly sent back as reported properties. Futhermore the module and azure-sdk version is reported as well as the current network adapter status.
3. **Direct methods**: [direct_methods.rs](src/direct_methods.rs) implements two functions that serve as direct method and can be synchronously called by iot-hub:
   1. `closure_send_d2c_message`: A closure that doesn't take a parameter and doesn't return a result. The method triggers an outgoing D2C message (@see **3. D2C message**).
   2. `func_echo_params_as_result`: A function that takes a parameter and returns the same parameter as result.
4. **Device to cloud messages (D2C)**: In [direct_methods.rs](src/direct_methods.rs) there is a direct method call named `closure_send_d2c_message`. It shows how to send a D2C telemetry event to iot-hub.
5. **Cloud to device messages (C2D)**: [message.rs](src/message.rs) demonstrates how the application receives messages sent from cloud. As an example the content of the received message is logged to the console. In order to test that functionality it is the easiest way to configure the application as `device_twin` and send a message from [iot-explorer](https://docs.microsoft.com/en-us/azure/iot-pnp/howto-use-iot-explorer).

All examples can be tested via [iot-explorer](https://docs.microsoft.com/en-us/azure/iot-pnp/howto-use-iot-explorer) or directly via iot-hub view in your Azure portal.

# Platform integration

Both of the supported omnect Device Management targets - [yocto](https://github.com/omnect/meta-omnect) and [simulator](https://github.com/omnect/simulator) - integrate the `iot-client-template-rs` and serve as an example for device integration.

# Client identity creation in Azure iot-hub

In order to enable the communication between client and cloud a device or module identity needs to be created in Azure iot-hub.<br>
***Note: This only applies to client types device_client and module_client (clients of type edge_client connect via edge runtime).***

1. **Client identity creation on device via Azure Identity Service (AIS)**: In case your device integrates [AIS](https://azure.github.io/iot-identity-service/), the module creation will be managed automatically on demand. Omnect Device Management yocto layer and simulator support AIS by default.
2. **Manual identity creation and connection string**: As an alternative you might create your device or modules manually in iot-hub and pass the corresponding connection string to the `client.run()` call in [lib.rs](src/lib.rs):

```
   client.run(
      Some("your connection string"),
      Some(methods),
      tx_client2app,
      rx_app2client,
   );
```
# License

Licensed under either of
* Apache License, Version 2.0, (./LICENSE-APACHE or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license (./LICENSE-MIT or <http://opensource.org/licenses/MIT>)
at your option.

# Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

Copyright (c) 2021 conplement AG
