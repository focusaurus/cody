FROM rust:1.27
RUN apt-get -q -y update && apt-get install less git-core
RUN \
  rustup install nightly && \
  rustup update && \
  rustup component add clippy-preview --toolchain=nightly
