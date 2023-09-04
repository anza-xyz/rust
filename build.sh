#!/usr/bin/env bash

set -ex

if [ "$1" == "--help" ] || [ "$1" == "-h" ]; then
    echo "--llvm to rebuild llvm";
    exit;
fi

unameOut="$(uname -s)-$(uname -m)"
case "${unameOut}" in
    FreeBSD-amd64*) HOST_TRIPLE=x86_64-unknown-freebsd;;
    Linux-x86_64*)  HOST_TRIPLE=x86_64-unknown-linux-gnu;;
    Linux-aarch64*) HOST_TRIPLE=aarch64-unknown-linux-gnu;;
    Darwin-x86_64*) HOST_TRIPLE=x86_64-apple-darwin;;
    Darwin-arm64*)  HOST_TRIPLE=aarch64-apple-darwin;;
    MINGW*)         HOST_TRIPLE=x86_64-pc-windows-msvc;;
    *)              HOST_TRIPLE=x86_64-unknown-linux-gnu
esac

if [[ "${HOST_TRIPLE}" == *-freebsd ]]; then
    if [[ "$(swig -swiglib)" == */4.1.* ]]; then
	# Default swig 4.1.2 on FreeBSD 13.x doesn't work due to
	# broken %nothreadallow / %clearnothreadallow directives
	# Use swig40 instead
	fgrep -wq 'SWIG_EXECUTABLE' config.toml ||
	ed config.toml <<-'EOF'
	/^\[llvm\]/a

	# Custom CMake defines to set when building LLVM.
	build-config = { SWIG_EXECUTABLE = '/usr/local/bin/swig40' }
	.
	+1,$g/^ *build-config *=/d
	w
	EOF
    fi
fi

if [ "$1" == "--llvm" ]; then
    rm -f build/${HOST_TRIPLE}/llvm/llvm-finished-building;
fi
./x.py build --stage 1 --target ${HOST_TRIPLE},sbf-solana-solana
