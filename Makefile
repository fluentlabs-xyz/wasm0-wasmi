CARGO_BINARY ?= cargo
CARGO_TARGET ?=
CARGO_TARGET_FLAG ?=

ifneq ($(CARGO_TARGET),)
CARGO_TARGET_FLAG := --target $(CARGO_TARGET)
endif

.PHONY: build-apple-amd64 build-apple-aarch64 build-linux-amd64 build generate-headers
build-apple-amd64:
	rustup target add x86_64-apple-darwin
	RUSTFLAGS="${RUSTFLAGS}" $(CARGO_BINARY) build --target=x86_64-apple-darwin --manifest-path ./crates/c-api/Cargo.toml --release #--no-default-features $(capi_compiler_features)
	mkdir -p packaged/lib/darwin-amd64/
	cp ./target/x86_64-apple-darwin/release/libwasmi_c_api.dylib packaged/lib/darwin-amd64/
build-apple-aarch64:
	rustup target add aarch64-apple-darwin
	RUSTFLAGS="${RUSTFLAGS}" $(CARGO_BINARY) build --target=aarch64-apple-darwin --manifest-path ./crates/c-api/Cargo.toml --release #--no-default-features $(capi_compiler_features)
	mkdir -p packaged/lib/darwin-aarch64/
	cp ./target/aarch64-apple-darwin/release/libwasmi_c_api.dylib packaged/lib/darwin-aarch64/
build-linux-amd64:
	rustup target add x86_64-unknown-linux-gnu
	RUSTFLAGS="${RUSTFLAGS}" $(CARGO_BINARY) build --target=x86_64-unknown-linux-gnu --manifest-path ./crates/c-api/Cargo.toml --release #--no-default-features $(capi_compiler_features)
	mkdir -p packaged/lib/linux-amd64/
	cp ./target/x86_64-unknown-linux-gnu/release/libwasmi_c_api.so packaged/lib/linux-amd64/

build: build-apple-amd64 build-apple-aarch64 build-linux-amd64

generate-headers:
	cargo run --package wasmi_c_api_crate --bin generate_headers --features=headers

clean:
	cargo clean
