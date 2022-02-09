# iot-module-template-rs

## What is iot-module-template-rs?
This `iot-module-template-rs` repository provides code to develop Rust based [Azure IoT modules](https://docs.microsoft.com/en-us/azure/iot-hub/iot-hub-devguide-module-twins) for ICS_DeviceManagement. In comparison to [IoTEdge modules](https://docs.microsoft.com/en-us/azure/iot-edge/module-development?view=iotedge-2020-11) these modules are not containerized and run natively on the device. In order to develop C/C++ based IoTEdge modules for ICS_DeviceManagement refer to the [iotedge-module-template](https://github.com/ICS-DeviceManagement/iotedge-module-template) repository.

## Template and example code
This project shows a basic skeleton for an initial implementation of a Rust based iot module. It demonstrates how to make use of our Rust [azure-iot-sdk](https://github.com/ICS-DeviceManagement/azure-iot-sdk) in order to connect to Azure iot-hub. Moreover there are examples to show basic communication patterns:
1. **Module twin**: There is some logic implemented that demonstrates how the module twin could be used by applications. Therefore the EventHandler trait is implemented in order to receive new desired properties which are directly sent back as reported property.
2. **Direct methods**: There are two functions implemented that serve as direct method and can be synchronously called by iot-hub:
   1. `closure_no_param_no_result`: A closure that doesn't take a parameter and doesn't return a result.
   2. `func_params_as_result`: A function tha takes a parameter and returns the same parameter as result.
3. **Cloud to device messages (C2D)** : Not implemented yet.
4. **Device to cloud messages (D2C)** : Not implemented yet.

All examples can be tested via module view in Azure portal or via [iot-explorer](https://docs.microsoft.com/en-us/azure/iot-pnp/howto-use-iot-explorer)).

## Platform integration
Both of the supported ICS_DeviceManagement targets - [yocto](https://github.com/ICS-DeviceManagement/meta-ics-dm) and [simulator](https://github.com/ICS-DeviceManagement/simulator) - integrate the iot-module-template-rs and serve as an example for device integration.

## Module identity creation in Azure iot-hub
In order to enable the communication between module and cloud a module identity needs to be created in Azure iot-hub.

### Module identity creation via Azure Identity Service (AIS)
In case your device provides [AIS](https://azure.github.io/iot-identity-service/), the module creation will be managed automatically on demand. ICS_DeviceManagement yocto layer and simulator support AIS by default.

### Module identity creation via powershell script
In case the device doesn't come with AIS you have to create module identity manually or by script support. For that purpose this repository provides a powershell [script](script/Add-ModuleToAllDevicesInIotHub.ps1) which creates a module identity for newly created devices. In order to automate that process these steps must be followed:
1. Create an Azure function powershell app either in portal or from [VSC](https://docs.microsoft.com/de-de/azure/azure-functions/create-first-function-vs-code-powershell).
2. [Create an event subscription](https://docs.microsoft.com/en-us/azure/iot-hub/iot-hub-event-grid) for the `Device Created` event in your iot-hub. Select the Azure function as endpoint.
3. Adapt the [script](script/Add-ModuleToAllDevicesInIotHub.ps1) to your needs:
   1. IOT_HUB_CONNECTION_STRING: set to your specific iot-hub connection string
   2. MODULE_NAME: set to your specific module name that will be used as identity
   3. Create TAGS and PROPERTIES in `CreateAndPopulateModuleForDevice` function in order to define initial module-twin settings

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