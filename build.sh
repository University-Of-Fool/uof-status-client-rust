#!/bin/env bash
# Build script for Github Action

# Build target
TARGET=(x86_64-unknown-linux-gnu x86_64-unknown-linux-musl x86_64-pc-windows-msvc x86_64-pc-windows-gnu)

# Check environment
hash cargo || exit 1 & echo "Please install cargo"
cargo install cross

# Start to build
mkdir output
for BUILD_TARGET in ${TARGET[@]}; do
	cross build --release --target=$BUILD_TARGET
	mv 

