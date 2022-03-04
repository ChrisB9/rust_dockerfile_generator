VERSION = $(patsubst "%",%, $(word 3, $(shell grep version Cargo.toml)))
BIN_NAME = dockerfile-generator

.PHONY: clean release

clean:
	cargo clean

release:
	cargo build --release --target=x86_64-unknown-linux-musl
	ls target
	ls target/x86_64-unknown-linux-musl
	ls target/x86_64-unknown-linux-musl/release
	zip -j dockerfile-generator-v${VERSION}-x86_64-lnx.zip target/x86_64-unknown-linux-musl/release/dockerfile-generator


