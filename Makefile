

build:
	cargo build

autobuild:
	while inotifywait -e close_write `find ./src`; do make build && make test; done

run:
	cargo run

test:
	cargo test

debug:
	rust-gdb -tui ./target/debug/nslfmt
