all: target/release/libconstellation.dylib gemtest

clean:
	cargo clean

gemtest: target/release/libconstellation.dylib
	bin/rake

target/release/libconstellation.dylib: rusttest
	cargo build --release

target/release/libconstellation.so: rusttest
	vagrant up
	vagrant ssh -c "cd /vagrant && cargo build --release"
	vagrant scp default:/vagrant/target ./target

rusttest:
	cargo test

image: all
	bin/test_image

docker: target/release/libconstellation.so
	docker build -t bhaibel/constellation_bot:`cat .docker-image-version` .

production: clean all docker

.PHONY: all gemtest clean rusttest image production docker