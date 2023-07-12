FROM rust

WORKDIR /build

COPY Cargo.toml Cargo.lock rust-toolchain /build/
RUN rustup show

COPY crates/arena/Cargo.toml /build/crates/arena/Cargo.toml
RUN mkdir -p /build/crates/arena/src && touch /build/crates/arena/src/lib.rs

COPY crates/c-api/Cargo.toml /build/crates/c-api/Cargo.toml
RUN mkdir -p /build/crates/c-api/src && touch /build/crates/c-api/src/lib.rs

COPY crates/bin/Cargo.toml /build/crates/cli/Cargo.toml
RUN mkdir -p /build/crates/bin/src && touch /build/crates/bin/src/main.rs

COPY crates/core/Cargo.toml /build/crates/core/Cargo.toml
RUN mkdir -p /build/crates/core/src && touch /build/crates/core/src/lib.rs

COPY crates/wasi/Cargo.toml /build/crates/wasi/Cargo.toml
RUN mkdir -p /build/crates/wasi/src && touch /build/crates/wasi/src/lib.rs

COPY crates/runtime/Cargo.toml /build/crates/wasmi/Cargo.toml
RUN mkdir -p /build/crates/runtime/src && touch /build/crates/runtime/src/lib.rs
RUN mkdir -p /build/crates/runtime/benches && touch /build/crates/runtime/benches/benches.rs

RUN cargo fetch --locked -v

CMD echo "specify a command to run. nothing todo"
