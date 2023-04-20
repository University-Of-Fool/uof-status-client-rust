#!/bin/bash
# Build script for batch cross-compilation

# Set build target
TARGET=(x86_64-unknown-linux-gnu x86_64-unknown-linux-musl x86_64-pc-windows-msvc x86_64-pc-windows-gnu)
OUTPUT_FILE=(uof-status uof-status.exe)

# Check environment
source "$HOME/.cargo/env"
hash cargo || exit 1 && echo "NOTICE:Please make sure \"cargo\" is installed!"
cargo install cross

# Build function
build() {
	cross build --release --target=$BUILD_TARGET
	mkdir output/$BUILD_TARGET
	for FILE in ${OUTPUT_FILE[@]}; do
		cp -rf target/$BUILD_TARGET/release/$FILE output/$BUILD_TARGET/$FILE
	done
}
build_() {
	cross build --release --target=$BUILD_TARGET_
	mkdir output/$BUILD_TARGET_
	for FILE in ${OUTPUT_FILE[@]}; do
		cp -rf target/$BUILD_TARGET_/release/$FILE output/$BUILD_TARGET_/$FILE
	done
}

# Start to build
mkdir output
for BUILD_TARGET in ${TARGET[@]}; do
	build &
done

# Extra build from arguments
for BUILD_TARGET_ in $*; do
	build_ &
done

wait