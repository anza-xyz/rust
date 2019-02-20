./#!/usr/bin/env bash

set -x

if [ "$1" == "--help" ] || [ "$1" == "-h" ]; then
    echo "--llvm to rebuild llvm";
    exit;
fi

if [ "$1" == "--llvm" ]; then
    rm build/x86_64-apple-darwin/llvm/llvm-finished-building;
fi
./x.py build --stage 1

# Not needed so save space
rm -rf build/x86_64-apple-darwin/stage1/lib/rustlib/x86_64-apple-darwin/lib



