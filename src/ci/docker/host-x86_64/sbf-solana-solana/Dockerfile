FROM ubuntu:20.04

RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
    g++ \
    make \
    ninja-build \
    file \
    curl \
    ca-certificates \
    python3 \
    git \
    cmake \
    sudo \
    gdb \
    libssl-dev \
    pkg-config \
    xz-utils

ENV RUSTUP_INIT_SKIP_PATH_CHECK="yes"
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

RUN PATH="${HOME}/.cargo/bin:${PATH}" \
    cargo install --git https://github.com/solana-labs/cargo-run-bpf-tests.git \
    --rev f0201b677976190568b8a04d8ede018fc6ccf053 \
    --bin cargo-run-bpf-tests --root /usr/local

COPY scripts/sccache.sh /scripts/
RUN sh /scripts/sccache.sh

ENV RUST_CONFIGURE_ARGS \
    --set rust.lld \
    --set llvm.clang

ENV SCRIPT CARGO_TARGET_BPFEL_UNKNOWN_UNKNOWN_RUNNER=\"cargo-run-bpf-tests --heap-size 104857600\" \
    CARGO_TARGET_SBF_SOLANA_SOLANA_RUNNER=\"cargo-run-bpf-tests --heap-size 104857600\" \
    LLVM_HOME=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm \
    PATH="${HOME}/.cargo/bin:${PATH}" \
    python3 /checkout/x.py --stage 1 test --host='' --target bpfel-unknown-unknown,sbf-solana-solana \
    library/core
