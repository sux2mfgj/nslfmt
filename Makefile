
build:
	cargo build

autobuild:
	while inotifywait -e close_write `find ./src`; do make build && make test; done

run:
	cargo run

test:
	RUST_BACKTRACE=1 cargo test

debug:
	rust-gdb -tui ./target/debug/nslfmt

fmt:
	cargo-fmt --all
