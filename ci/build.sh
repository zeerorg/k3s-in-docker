#!/bin/sh

if [[ "$(uname -s)" != "Linux" ]]
then
  exit 0
fi

# Set linker environment variables
export CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABIHF_LINKER=arm-linux-gnueabihf-gcc
export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc
export CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER=x86_64-w64-mingw32-gcc

targets=("x86_64-pc-windows-gnu" "x86_64-unknown-linux-gnu" "armv7-unknown-linux-gnueabihf" "aarch64-unknown-linux-gnu")
for trgt in ${targets[*]}
do
  cargo build --target $trgt --verbose
  if [ "$?" != "0" ]
  then
    exit 1
  fi
done