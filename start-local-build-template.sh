#!/bin/sh

export LIB_PATH_AZURESDK=<path to the azure iot sdk c >
export LIB_PATH_EISUTILS=<path to eis utils >
export LIB_PATH_UUID=<path to uid >
export LIB_PATH_OPENSSL=<path to openssl >
export LIB_PATH_CURL=<path to curl>

cargo build
