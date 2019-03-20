#!/bin/sh

# Set linker environment variables
export CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABIHF_LINKER=arm-linux-gnueabihf-gcc
export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc
export CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER=x86_64-w64-mingw32-gcc

cargo build --verbose --target x86_64-pc-windows-gnu &
cargo build --verbode --target x86_64-unknown-linux-gnu &
cargo build --verbose --target armv7-unknown-linux-gnueabihf &
cargo build --verbose --target aarch64-unknown-linux-gnu &

wait