
build:
	cargo build

autobuild:
	while inotifywait -e close_write `find ./src`; do make build && make test; done

run:
	cargo run

test:
	cargo test

details:
	RUST_BACKTRACE=1 cargo test

debug:
	rust-gdb -tui ./target/debug/nslfmt

debug_test:
	rust-gdb -tui $(shell ls -t target/debug/nslfmt-* |head -n 1)

fmt:
	cargo-fmt --all
