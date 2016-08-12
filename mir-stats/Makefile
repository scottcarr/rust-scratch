RC=~/mozilla/rust-workspace/mir-bench/rust/x86_64-apple-darwin/stage1/bin/rustc
LOG=RUST_LOG=rustc_mir::transform::mir_stats

all: return_an_array

return_an_array: return_an_array.rs
	$(LOG) $(RC) -Z dump-mir=foo -Z print-mir-stats return_an_array.rs 2>&1 | tee return_an_array.log


clean:
	rm return_an_array rustc.node* *.log
