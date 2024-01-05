FROM arm32v7/rust:buster

RUN apt-get update && apt-get install -y \
    gcc-arm-none-eabi \
    build-essential \
    llvm \ 
    clang

RUN rustup toolchain install nightly && rustup default nightly

RUN rustup target add thumbv7em-none-eabihf

ENV CARGO_NET_GIT_FETCH_WITH_CLI=true

ENV CARGO_HOME=/app/.cargo