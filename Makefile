

build:
	cargo build

autobuild:
	while inotifywait -e close_write src/main.rs; do make build && make test; done

run:
	cargo run

test:
	cargo test

debug:
	rust-gdb -tui ./target/debug/nslfmt
