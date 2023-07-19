#!/usr/bin/env bash
# Run from the script's directory.
cd -- "$(dirname -- "${BASH_SOURCE[0]}")"

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
# --out-dir is unstable
cp ./target/x86_64-unknown-linux-musl/release/shortpath ./shortpath

# Strip the binary and compress it with UPX.
strip --strip-all ./shortpath
upx --best ./shortpath
ls -lah "$(readlink -f ./shortpath)"

# Check that the binary is statically linked.
file ./shortpath | grep 'static.* linked'
