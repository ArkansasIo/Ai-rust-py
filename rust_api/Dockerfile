FROM rust:1.89-bookworm AS builder
WORKDIR /app
COPY Cargo.toml .
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/ai_ml_rust_api /usr/local/bin/ai_ml_rust_api
EXPOSE 8080
CMD ["/usr/local/bin/ai_ml_rust_api"]
