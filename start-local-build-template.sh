#!/bin/sh

# Hint: Both of the supported ICS_DeviceManagement targets
# [yocto](https://github.com/ICS-DeviceManagement/ics-dm-os) and
# [simulator](https://github.com/ICS-DeviceManagement/simulator)
# integrate the ics-dm-iot-module-rs.

# If you like to built it independently the paths from the libraries listed below are required.
# Copy `.start-local-build-template.sh` to `.start-local-build.sh` and adapt it.
# Example:
# export LIB_PATH_AZURESDK=/home/user/projects/GitHub/simulator/build/.conan/data/azure-iot-sdk-c/LTS_07_2021_Ref01/_/_/package/3bf7811c9395d29095bf663023235996901b6af2
# export LIB_PATH_EISUTILS=/home/user/projects/GitHub/simulator/build/.conan/data/libeis_utils/0.7.0/_/_/package/*

#export LIB_PATH_AZURESDK=/home/osboxes/projects/simulator/build/.conan/data/azure-iot-sdk-c/LTS_07_2021_Ref01/_/_/package/10a9b29f5f7ce50f3eeb346ff718b962a5264367
export LIB_PATH_AZURESDK=/home/osboxes/projects/simulator/build/.conan/data/azure-iot-sdk-c/LTS_07_2021_Ref01/_/_/package/3bf7811c9395d29095bf663023235996901b6af2
export LIB_PATH_UUID=/home/osboxes/projects/simulator/build/.conan/data/libuuid/1.0.3/_/_/package/c9f99894de7cd0d22d0bdedcf23221ef3d5c6f04
export LIB_PATH_OPENSSL=/home/osboxes/projects/simulator/build/.conan/data/openssl/1.1.1i/_/_/package/c9f99894de7cd0d22d0bdedcf23221ef3d5c6f04
export LIB_PATH_CURL=/home/osboxes/projects/simulator/build/.conan/data/libcurl/7.72.0/_/_/package/1c2c08939ea21ab857f6c9eae7f3501187cb8811

cargo build
