# SPDX-FileCopyrightText: 2025 Anza Technology Inc. <https://www.anza.xyz>
#
# SPDX-License-Identifier: MIT

#!/usr/bin/env bash

set -ex

if [ "$1" == "--help" ] || [ "$1" == "-h" ]; then
    echo "--llvm to rebuild llvm";
    exit;
fi

unameOut="$(uname -s)-$(uname -m)"
case "${unameOut}" in
    Linux-x86_64*)  HOST_TRIPLE=x86_64-unknown-linux-gnu;;
    Linux-aarch64*) HOST_TRIPLE=aarch64-unknown-linux-gnu;;
    Darwin-x86_64*) HOST_TRIPLE=x86_64-apple-darwin;;
    Darwin-arm64*)  HOST_TRIPLE=aarch64-apple-darwin;;
    MINGW*)         HOST_TRIPLE=x86_64-pc-windows-msvc;;
    *)              HOST_TRIPLE=x86_64-unknown-linux-gnu
esac

if [ "$1" == "--llvm" ]; then
    rm -f build/${HOST_TRIPLE}/llvm/llvm-finished-building;
fi
./x.py build --stage 1 --target ${HOST_TRIPLE},sbf-solana-solana,sbpf-solana-solana,sbpfv1-solana-solana,sbpfv2-solana-solana,sbpfv3-solana-solana,sbpfv4-solana-solana
