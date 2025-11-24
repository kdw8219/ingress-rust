FROM rust:1.72 as builder
WORKDIR /usr/src/ingress
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
# copy built binary
COPY --from=builder /usr/src/ingress/target/release/ingress-rust /usr/local/bin/ingress-rust
# copy entrypoint script
COPY entrypoint.sh /usr/local/bin/entrypoint.sh
RUN chmod +x /usr/local/bin/entrypoint.sh

EXPOSE 0
ENTRYPOINT ["/usr/local/bin/entrypoint.sh"]
