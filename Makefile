CARGO_BINARY ?= cargo
CARGO_TARGET ?=
CARGO_TARGET_FLAG ?=

ifneq ($(CARGO_TARGET),)
CARGO_TARGET_FLAG := --target $(CARGO_TARGET)
endif

.PHONY: build-apple-amd64 build-apple-aarch64 build-linux-amd64 build run test clean
build-apple-amd64:
	rustup target add x86_64-apple-darwin
	RUSTFLAGS="${RUSTFLAGS}" $(CARGO_BINARY) build --target=x86_64-apple-darwin --manifest-path ./Cargo.toml --release #--no-default-features $(capi_compiler_features)
	#cp wasm-instrument-c-api/target/x86_64-apple-darwin/release/libgas_injector.dylib packaged/lib/darwin-amd64/
build-apple-aarch64:
	rustup target add aarch64-apple-darwin
	RUSTFLAGS="${RUSTFLAGS}" $(CARGO_BINARY) build --target=aarch64-apple-darwin --manifest-path wasm-instrument-c-api/Cargo.toml --release #--no-default-features $(capi_compiler_features)
	#cp wasm-instrument-c-api/target/aarch64-apple-darwin/release/libgas_injector.dylib packaged/lib/darwin-aarch64/
build-linux-amd64:
	rustup target add x86_64-unknown-linux-gnu
	RUSTFLAGS="${RUSTFLAGS}" $(CARGO_BINARY) build --target=x86_64-unknown-linux-gnu --manifest-path wasm-instrument-c-api/Cargo.toml --release #--no-default-features $(capi_compiler_features)
	cp wasm-instrument-c-api/target/x86_64-unknown-linux-gnu/release/libgas_injector.so packaged/lib/linux-amd64/