#!/bin/sh

if [[ "$(uname -s)" != "Linux" ]]
then
  exit 0
fi

# Install cross building compilers
sudo apt-get update
sudo apt-get install -y --no-install-recommends g++-mingw-w64-x86-64 g++-aarch64-linux-gnu libc6-dev-arm64-cross g++-arm-linux-gnueabihf libc6-dev-armhf-cross

rustup target add x86_64-pc-windows-musl &
rustup target add armv7-unknown-linux-musleabihf &
rustup target add aarch64-unknown-linux-gnu &

wait