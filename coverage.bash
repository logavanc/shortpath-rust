#!/usr/bin/env bash
set -e

################################################################################
# See: https://blog.rng0.io/how-to-do-code-coverage-in-rust
# This script is used to generate the Rust unit test coverage lcov files.
# You will need to have the following installed:
#
# - rustup component add llvm-tools-preview
# - cargo install grcov
#
# VS Code Integration
# Install the workspace extension recommendations. Appropriate settings have
# been added to the workspace settings.json file.
# MAKE SURE TO ENABLE THE "COVERAGE GUTTERS" WATCHER!
#
# Don't forget to add the /coverage/ directory to your .gitignore file,
# and I would also recommend excluding profraw files from VS Code.
################################################################################

# Run from the script's directory.
cd -- "$(dirname -- "${BASH_SOURCE[0]}")"

# Setup the output directory for the lcov files.
mkdir -p coverage

# Run the tests and generate the profraw files.
echo "Running tests..."
CARGO_INCREMENTAL=0 \
    RUSTFLAGS='-Cinstrument-coverage' \
    LLVM_PROFILE_FILE='coverage/cargo-test-%p-%m.profraw' \
    cargo test

# Generate the lcov files using the grcov tool and the profraw files.
echo "Generating lcov files..."
grcov . \
    --binary-path "./target/debug/deps" \
    --source-dir . \
    --output-type lcov \
    --branch \
    --ignore-not-existing \
    --ignore "*_test.rs" \
    -o coverage/tests.lcov
