#!/usr/bin/env bash

# Bash 'Strict Mode'
# http://redsymbol.net/articles/unofficial-bash-strict-mode
# https://github.com/xwmx/bash-boilerplate#bash-strict-mode
set -o nounset
set -o errexit
set -o pipefail
IFS=$'\n\t'

export RUSTFLAGS="-C target-cpu=native"

cargo build \
	--target=x86_64-unknown-linux-musl \
	--release

file target/x86_64-unknown-linux-musl/release/shortpath |
	grep 'static.* linked'
