FROM rust
WORKDIR /build
ADD . .
RUN make build-linux-amd64