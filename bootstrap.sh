#!/usr/bin/env bash

apt-get update && \
    apt-get install \
       ca-certificates \
       curl \
       gcc \
       libc6-dev \
       -qqy \
       --no-install-recommends \
    && rm -rf /var/lib/apt/lists/*

export RUST_ARCHIVE=rust-beta-x86_64-unknown-linux-gnu.tar.gz
export RUST_DOWNLOAD_URL=https://static.rust-lang.org/dist/$RUST_ARCHIVE

mkdir /rust
cd /rust \
    && curl -fsOSL $RUST_DOWNLOAD_URL \
    && curl -s $RUST_DOWNLOAD_URL.sha256 | sha256sum -c - \
    && tar -C /rust -xzf $RUST_ARCHIVE --strip-components=1 \
    && rm $RUST_ARCHIVE \
    && ./install.sh