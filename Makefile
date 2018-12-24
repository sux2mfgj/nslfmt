TEST_TARGET			:= parser
LOCAL_TEST_TARGET	:= # module::wire_0
SRC_TESTS			:= $(shell find ./src) $(shell find ./tests)
INOTIFY				:= inotifywait -e close_write $(SRC_TESTS)

build:
	cargo build

autobuild:
	while $(INOTIFY); do make build && make test; done

auto_unittest:
	while $(INOTIFY); do make build && make test_module TEST_TARGET=$(TEST_TARGET); done

test_local_auto:
	while $(INOTIFY); do make build && make test_local TEST_TARGET=$(TEST_TARGET) LOCAL_TEST_TARGET=$(LOCAL_TEST_TARGET); done

run:
	cargo run

test:
	cargo test

test_module:
	cargo test --test $(TEST_TARGET)

test_local:
	cargo test $(LOCAL_TEST_TARGET) --test $(TEST_TARGET)

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
