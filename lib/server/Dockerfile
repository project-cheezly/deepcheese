FROM rust:1.77-bookworm AS builder

WORKDIR /usr/src

COPY . .

WORKDIR /usr/src/server
RUN cargo build --release

FROM rust:1.77-bookworm

WORKDIR /usr/local/bin

COPY --from=builder /usr/src/server/target/release/server .

CMD ["./server"]