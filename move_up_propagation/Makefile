RC=~/mozilla/rust-workspace/move-up-propagation/rust/x86_64-apple-darwin/stage1/bin/rustc
LOG=RUST_LOG=rustc_mir::transform::move_up_propagation

all: return_an_array

return_an_array: return_an_array.rs
	$(LOG) $(RC) -Z orbit -Z dump-mir=foo return_an_array.rs 2>&1 | tee return_an_array.log


clean:
	rm return_an_array rustc.node* *.log
