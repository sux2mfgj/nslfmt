TEST_TARGET=lexer_test

build:
	cargo build

autobuild:
	while inotifywait -e close_write `find ./{src,tests}`; do make build && make test; done

auto_unittest:
	while inotifywait -e close_write `find ./{src,tests}`; do make build && make test_module TEST_TARGET=$(TEST_TARGET); done

run:
	cargo run

test:
	cargo test

test_module:
	cargo test --test $(TEST_TARGET)

coverage:
	rustup run nightly cargo tarpaulin -v

details:
	RUST_BACKTRACE=1 cargo test

debug:
	rust-gdb -tui ./target/debug/nslfmt

debug_test:
	rust-gdb -tui $(shell ls -t target/debug/nslfmt-* |head -n 1)

fmt:
	cargo-fmt --all
