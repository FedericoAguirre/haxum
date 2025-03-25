.PHONY: fmt test run build clean release exec

fmt:
	cargo fmt

test: fmt
	cargo test

run: test
	cargo run

build: test
	cargo build

clean:
	cargo clean

release: test
	cargo build --release --color=always

exec: release
	./target/release/$(shell basename $(CURDIR))
