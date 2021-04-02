#!/usr/bin/env bash

set -ex

if [ "$1" == "--help" ] || [ "$1" == "-h" ]; then
    echo "--llvm to rebuild llvm";
    exit;
fi

if [ "$(uname)" == "Darwin" ]; then
    HOST_TRIPLE=x86_64-apple-darwin
else
    HOST_TRIPLE=x86_64-unknown-linux-gnu
fi

if [ "$1" == "--llvm" ]; then
    rm -f build/x86_64-apple-darwin/llvm/llvm-finished-building;
fi
./x.py build --stage 1 --target ${HOST_TRIPLE},bpfel-unknown-unknown
