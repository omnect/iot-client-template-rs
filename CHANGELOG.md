# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
