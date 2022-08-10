.PHONY: test_linux build_linux test_osx build_osx

test_linux:
	cross test --target x86_64-unknown-linux-musl

build_linux:
	cross build --release --target x86_64-unknown-linux-musl

test_osx:
	cargo test

build_osx:
	MACOSX_DEPLOYMENT_TARGET=10.7 cargo build --release
