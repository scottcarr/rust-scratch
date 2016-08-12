RUST_LOG=rustc_mir::transform::move_up_propagation \
RUSTC=~/mozilla/rust-workspace/move-up-propagation/rust/x86_64-apple-darwin/stage1/bin/rustc \
RUSTFLAGS="-Z orbit " \
cargo bench --verbose
