#!/bin/bash

docker run --rm -it -v "$(pwd)":"/home/rust/src" ekidd/rust-musl-builder:1.51.0 \
    cargo build --release --target x86_64-unknown-linux-musl

strip target/x86_64-unknown-linux-musl/release/contact-api
