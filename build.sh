# SPDX-FileCopyrightText: 2025 Anza Technology Inc. <https://www.anza.xyz>
#
# SPDX-License-Identifier: MIT

#!/usr/bin/env bash

set -ex

WITH_NIX=
REBUILD_LLVM=
while [ -n "$1" ]; do
    case "$1" in
        --nix)
            WITH_NIX=1
            shift
            ;;
        --llvm)
            REBUILD_LLVM=1
            shift
            ;;
        --help)
            echo "--llvm to rebuild llvm, --nix to use nix";
            exit;
    esac
done

unameOut="$(uname -s)-$(uname -m)"
case "${unameOut}" in
    Linux-x86_64*)  HOST_TRIPLE=x86_64-unknown-linux-gnu;;
    Linux-aarch64*) HOST_TRIPLE=aarch64-unknown-linux-gnu;;
    Darwin-x86_64*) HOST_TRIPLE=x86_64-apple-darwin;;
    Darwin-arm64*)  HOST_TRIPLE=aarch64-apple-darwin;;
    MINGW*)         HOST_TRIPLE=x86_64-pc-windows-msvc;;
    *)              HOST_TRIPLE=x86_64-unknown-linux-gnu
esac

if [ -n "${REBUILD_LLVM}" ]; then
    rm -f build/${HOST_TRIPLE}/llvm/llvm-finished-building;
fi

if [ -n "${WITH_NIX}" ]; then
    nix-shell src/tools/nix-dev-shell/shell.nix --pure --run "x build --stage 1 --target sbf-solana-solana,sbpf-solana-solana,sbpfv1-solana-solana,sbpfv2-solana-solana,sbpfv3-solana-solana,sbpfv4-solana-solana"
else
    ./x.py build --stage 1 --target sbf-solana-solana,sbpf-solana-solana,sbpfv1-solana-solana,sbpfv2-solana-solana,sbpfv3-solana-solana,sbpfv4-solana-solana
fi
