RC=~/mozilla/rust-workspace/discriminant/rust/x86_64-apple-darwin/stage1/bin/rustc
LOG=RUST_LOG=rustc_mir::transform::deaggregator,rustc_trans::mir::statement

all: main

main: main.rs
	RUST_BACKTRACE=1 $(LOG) $(RC) -Z orbit -Z mir-opt-level=3 -Z dump-mir=bar main.rs 2>&1 | tee main.log
	#$(LOG) $(RC) -Z orbit -Z dump-mir=bar main.rs 2>&1 | tee main.log


clean:
	rm main rustc.node* *.log
