.PHONY: test_linux build_linux test_osx build_osx
.SHELL: /bin/bash

NCPUS := $(shell if [ $(shell nproc) -gt 1 ]; then expr $(shell nproc) - 1; else 1; fi)

test_linux:
	CARGO_BUILD_JOBS=${NCPUS} cross test --target x86_64-unknown-linux-musl

build_linux:
	CARGO_BUILD_JOBS=${NCPUS} cross build --release --target x86_64-unknown-linux-musl

test_osx:
	CARGO_BUILD_JOBS=${NCPUS} cargo test

build_osx:
	CARGO_BUILD_JOBS=${NCPUS} MACOSX_DEPLOYMENT_TARGET=10.7 cargo build --release --target x86_64-apple-darwin

