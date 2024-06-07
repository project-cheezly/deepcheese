FROM --platform=$BUILDPLATFORM rust:1.77-bookworm AS builder

ARG TARGETPLATFORM
ARG BUILDPLATFORM

WORKDIR /usr/src

COPY . .

WORKDIR /usr/src/server
RUN apt-get update && apt-get upgrade -y && apt-get install -y gcc-aarch64-linux-gnu
RUN rustup target add aarch64-unknown-linux-gnu
RUN cargo build --release --target aarch64-unknown-linux-gnu

FROM --platform=$TARGETPLATFORM rust:1.77-bookworm

WORKDIR /usr/local/bin

COPY --from=builder /usr/src/server/target/aarch64-unknown-linux-gnu/release/server .

CMD ["./server"]