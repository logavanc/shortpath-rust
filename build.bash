export RUSTFLAGS="-C target-cpu=native"

cargo build \
    --target=x86_64-unknown-linux-musl \
    --release
