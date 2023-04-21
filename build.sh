#!/bin/bash
# Build script for batch cross-compilation

# Set build target
TARGET=("x86_64-unknown-linux-gnu --no-default-features" "x86_64-unknown-linux-musl --no-default-features" "x86_64-pc-windows-gnu --no-default-features" "x86_64-pc-windows-msvc --no-default-features" "aarch64-unknown-linux-gnu --no-default-features" "aarch64-unknown-linux-musl --no-default-features" "aarch64-pc-windows-gnu --no-default-features" "aarch64-pc-windows-msvc --no-default-features")
OUTPUT_FILE=(ustatc ustatc.exe)

# Check environment
source "$HOME/.cargo/env"
hash cargo || exit 1 && echo "NOTICE:Please make sure \"cargo\" is installed!"
cargo install cross

# Start to build
mkdir output
for BUILD_TARGET in ${TARGET[@]}; do
	cross build --release --target=$BUILD_TARGET
	mkdir output/$BUILD_TARGET
	for FILE in ${OUTPUT_FILE[@]}; do
		cp -rf target/$BUILD_TARGET/release/$FILE output/$BUILD_TARGET/$FILE
	done
done

# Extra build from arguments
for BUILD_TARGET_ in $*; do
	cross build --release --target=$BUILD_TARGET_
	mkdir output/$BUILD_TARGET_
	for FILE in ${OUTPUT_FILE[@]}; do
		cp -rf target/$BUILD_TARGET_/release/$FILE output/$BUILD_TARGET_/$FILE
	done
done
