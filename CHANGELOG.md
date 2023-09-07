# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.5.2] Q3 2023
 - removed module_client feature from azure-iot-sdk to be able to reconfigure this dependency

## [0.5.1] Q3 2023
 - fixed cargo clippy warnings

## [0.5.0] Q3 2023
 - replaced channel/thread based event dispatching by async dispatching based on azure-iot-sdk 0.10.0

## [0.4.21] Q2 2023
 - fixed RUSTSEC-2023-0044

## [0.4.20] Q2 2023
 - updated to azure-iot-sdk 0.9.5
 - changed omnect git dependencies from ssh to https url's

## [0.4.19] Q2 2023
 - removed rust-toolchain.toml since component is not used on device anymore.
   further the toolchain version 1.62 caused a bug when building for arm64v8.

## [0.4.18] Q2 2023
 - fixed RUSTSEC-2023-0034 (explicit `cargo update`)

## [0.4.17] Q1 2023
 - fixed GHSA-4q83-7cq4-p6wg (explicit `cargo update`)

## [0.4.16] Q1 2023
 - fixed redundant locking on twin
 - fixed redundant locking on iot messages

## [0.4.15] Q1 2023
 - simplified twin singleton
 - prepared readme for open sourcing repository

## [0.4.14] Q1 2023
 - fixed bug when application didn't exit on Unauthenticated message

## [0.4.13] Q1 2023
 - bumped to azure-iot-sdk 0.9.0
 - switched to anyhow based errors
 - twin:
   - refactored from functions to struct
   - wrapped as singleton

## [0.4.12] Q1 2023
 - updated tokio to 1.23 in order to fix cargo audit warning

## [0.4.11] Q4 2022
 - cleaned up Readme regarding ICS-DeviceManagement to omnect

## [0.4.10] Q4 2022
 - renamed from ICS-DeviceManagement to omnect github orga

## [0.4.9] Q3 2022
 - fixed bug when async client does not terminate correctly
 - improved logging for AuthenticationStatus changes

## [0.4.8] Q3 2022
 - log message with severity error on panics

## [0.4.7] Q3 2022
 - fixed report azure-sdk-version in twin
 - switched from forked sd-notify to new official release 0.4.1
 - changed some debug messages to log level info

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
