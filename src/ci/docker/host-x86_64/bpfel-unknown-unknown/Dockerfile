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
    xz-utils \
    git \
    cargo

RUN git clone https://github.com/solana-labs/cargo-run-bpf-tests
WORKDIR /cargo-run-bpf-tests
RUN cargo build
RUN cp target/debug/cargo-run-bpf-tests /usr/local/bin
WORKDIR /

COPY scripts/sccache.sh /scripts/
RUN sh /scripts/sccache.sh

ENV RUST_CONFIGURE_ARGS \
    --set rust.lld \
    --set llvm.clang

ENV SCRIPT CARGO_TARGET_BPFEL_UNKNOWN_UNKNOWN_RUNNER=cargo-run-bpf-tests \
    LLVM_HOME=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm \
    python3 /checkout/x.py --stage 1 test --host='' --target bpfel-unknown-unknown \
    library/core
