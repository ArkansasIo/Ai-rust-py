FROM rust:1.89-bookworm AS builder
WORKDIR /workspace
COPY rust_api/Cargo.toml rust_api/Cargo.toml
COPY rust_api/src rust_api/src
COPY database/migrations database/migrations
WORKDIR /workspace/rust_api
RUN cargo build --release
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /workspace/rust_api/target/release/ai_ml_rust_api /usr/local/bin/ai_ml_rust_api
EXPOSE 8080
CMD ["/usr/local/bin/ai_ml_rust_api"]
