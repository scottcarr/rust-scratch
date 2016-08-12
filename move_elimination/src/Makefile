RC=~/mozilla/rust-workspace/rust/x86_64-apple-darwin/stage1/bin/rustc
#RC=~/mozilla/rust-workspace/baseline/rust/x86_64-apple-darwin/stage1/bin/rustc
LOG=RUST_LOG=rustc_mir::build,rustc_mir::transform

#all: main issue-33567 issue-33567-2
#all: issue-33567-2
all: main

main: main.rs
	$(LOG) $(RC) -Z dump-mir=main main.rs 2>&1 | tee main.log

issue-33567: issue-33567.rs
	$(LOG) $(RC) -Z dump-mir=compare issue-33567.rs 2>&1 | tee issue-33567.log

issue-33567-2: issue-33567-2.rs
	$(LOG) $(RC) -Z dump-mir=compare issue-33567-2.rs 2>&1 | tee issue-33567-2.log

clean:
	rm main rustc.node* rust.log issue-33567 issue-33567-2 *.log
