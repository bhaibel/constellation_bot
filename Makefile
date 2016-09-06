all: target/release/libconstellation_bot.dylib gemtest

clean:
	cargo clean

gemtest: target/release/libconstellation_bot.dylib
	bin/rake

target/release/libconstellation_bot.dylib: rusttest
	cargo build --release

rusttest:
	cargo test

deps:
	bundle

.PHONY: all gemtest clean rusttest deps