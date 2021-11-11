#!/bin/sh

export LIB_PATH_AZURESDK=/home/joerg/projects/GitHub/simulator/build/.conan/data/azure-iot-sdk-c/LTS_07_2021_Ref01/_/_/package/3bf7811c9395d29095bf663023235996901b6af2
export LIB_PATH_EISUTILS=/home/joerg/projects/GitHub/simulator/build/.conan/data/libeis_utils/0.7.0/_/_/package/*
export LIB_PATH_UUID=/home/joerg/projects/GitHub/simulator/build/.conan/data/libuuid/1.0.3/_/_/package/*
export LIB_PATH_OPENSSL=/home/joerg/projects/GitHub/simulator/build/.conan/data/openssl/1.1.1i/_/_/package/*
export LIB_PATH_CURL=/home/joerg/projects/GitHub/simulator/build/.conan/data/libcurl/7.72.0/_/_/package/*

cargo build
