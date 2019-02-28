#!/usr/bin/env bash

set -ex

if [ "$1" == "--help" ] || [ "$1" == "-h" ]; then
    echo "--llvm to rebuild llvm";
    exit;
fi

if [ "$1" == "--llvm" ]; then
    rm build/x86_64-apple-darwin/llvm/llvm-finished-building;
fi
./x.py build --stage 1
