#!/usr/bin/env bash

# Bash 'Strict Mode'
# http://redsymbol.net/articles/unofficial-bash-strict-mode
# https://github.com/xwmx/bash-boilerplate#bash-strict-mode
set -o nounset
set -o errexit
set -o pipefail
IFS=$'\n\t'

# Required packages:
# rust (rustup)
# x86_64-unknown-linux-musl (rustup target add x86_64-unknown-linux-musl)
# upx
# strip (binutils)

# Build the static binary and make it as small and optimized as possible:
export RUSTFLAGS="-C target-cpu=native"
cargo build \
	--target=x86_64-unknown-linux-musl \
	--release

# Strip the binary and compress it with UPX.
strip --strip-all target/x86_64-unknown-linux-musl/release/shortpath
upx --best target/x86_64-unknown-linux-musl/release/shortpath

# Check that the binary is statically linked.
file target/x86_64-unknown-linux-musl/release/shortpath |
	grep 'static.* linked'
