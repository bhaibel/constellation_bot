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

production-deps:
	bundle install --without test development

image: all
	bin/test_image

production: clean production-deps
	cargo build --release -v # --target x86_64-linux-gnu

.PHONY: all gemtest clean rusttest deps image production