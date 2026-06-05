FROM rust:1.75-slim AS builder

WORKDIR /usr/src/runetalk

COPY Cargo.toml Cargo.lock ./

RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -f src/main.rs target/release/deps/runetalk*

COPY src ./src

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/runetalk/target/release/runetalk /app/runetalk

EXPOSE 6000

CMD ["./runetalk"]
