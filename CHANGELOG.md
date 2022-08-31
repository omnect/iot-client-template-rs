# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.7] Q3 2022
 - fix report azure-sdk-version in twin

## [0.4.6] Q3 2022
 - report azure-sdk-version in twin
 - log info message for azure-sdk-version
 - bump to azure-iot-sdk 0.8.4

## [0.4.5] Q3 2022
 - start service after time-sync target to avoid time jumps during service start
 - added info message for logging the package version

## [0.4.4] Q3 2022
 - fixed panic when receiving unauthenticated message

## [0.4.3] Q3 2022
 - fixed panic when calling IotHubClient::from_identity_service
 - fixed terminating on ExpiredSasToken
 - bumped to latest azure-iot-sdk 0.8.3

## [0.4.2] Q3 2022
 - bump to azure-iot-sdk 0.8.2

## [0.4.1] Q2 2022
 - fix build with "edge_client" feature

## [0.4.0] Q2 2022
 - replaced std::thread by tokio
 - added version reporting to twin

## [0.3.2] Q2 2022
 - bump to azure-iot-sdk 0.8.1

## [0.3.1] Q2 2022
 - fixed readme

## [0.3.0] Q2 2022
 - bump to azure-iot-sdk 0.8.0:
   - extend sample to demonstrate iot edge module feature

## [0.2.4] Q2 2022
 - improved build documentation in readme
 - added Cargo.audit.ignore
 - update to azure-iot-sdk 0.5.5

## [0.2.3] Q1 2022
 - add device twin example
 - renamed package and repo to iot-client-template-rs

## [0.2.2] Q1 2022
 - add optional systemd integration
   - notify ready
   - notify watchdog

## [0.2.1] Q1 2022
 - add D2C messages

## [0.2.0] Q1 2022
 - renamed package and repo to iot-module-template-rs
 - updated to latest azure-iot-sdk
 - added module twin example
 - added direct method example
 - Cargo.toml: added SPDX license expression

## [0.1.3] Q4 2021
 - add restart limit for system service file
 - restart failed "ics-dm-iot-module-rs service" after 10 min

## [0.1.2] Q4 2021
 - set repository in Cargo.toml (enables cargo-bitbake to generate yocto recipe)

## [0.1.1] Q4 2021
 - use unique user account

## [0.1.0] Q4 2021
 - initial version
