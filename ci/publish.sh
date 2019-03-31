#!/bin/sh

rustup target add $TARGET

case $TARGET in
"x86_64-pc-windows-gnu")
  sudo apt-get update
  sudo apt-get install -y --no-install-recommends g++-mingw-w64-x86-64
  rustup target add x86_64-pc-windows-gnu
  export FILENAME="k3d.exe"
  ;;

"x86_64-unknown-linux-musl")
  export FILENAME="k3d"
  ;;

"armv7-unknown-linux-musleabihf")
  sudo apt-get update
  sudo apt-get install -y --no-install-recommends g++-arm-linux-gnueabihf libc6-dev-armhf-cross
  export FILENAME="k3d-armhf"
  ;;

"aarch64-unknown-linux-gnu")
  sudo apt-get update
  sudo apt-get install -y --no-install-recommends g++-aarch64-linux-gnu libc6-dev-arm64-cross
  export FILENAME="k3d-arm64"
  ;;

"x86_64-apple-darwin")
  export FILENAME="k3d-darwin"
  ;;
esac

cargo build --release --verbose --target $TARGET
sha256sum target/$TARGET/release/$FILENAME > target/$TARGET/release/$FILENAME.sha256
mkdir -p target/releases/
cp target/$TARGET/release/$FILENAME target/releases/
cp target/$TARGET/release/$FILENAME.sha256 target/releases/
