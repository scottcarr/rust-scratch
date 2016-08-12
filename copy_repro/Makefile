RC=~/mozilla/rust-workspace/move-up-propagation/rust/x86_64-apple-darwin/stage1/bin/rustc
LOG=RUST_LOG=rustc_mir::transform::move_up_propagation

all: main

main: main.rs
	$(LOG) $(RC) -Z orbit -Z dump-mir=test main.rs 2>&1 | tee main.log


clean:
	rm return_an_array rustc.node* *.log
