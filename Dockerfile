# Based on https://github.com/rust-lang-nursery/docker-rust-nightly/blob/master/nightly/Dockerfile
FROM buildpack-deps:stretch

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:/opt/target/debug:$PATH
WORKDIR /opt
RUN set -eux; \
    url="https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-gnu/rustup-init"; \
    wget --quiet "$url"; \
    chmod +x rustup-init;
RUN ./rustup-init -y --no-modify-path --default-toolchain stable-2020-02-27
RUN set -eux; \
    rm rustup-init;
USER 1000:1000
