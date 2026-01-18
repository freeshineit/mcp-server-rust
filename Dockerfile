# Dockerfile
FROM rust:1.92 as builder

WORKDIR ./mcp-server-rust
COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder ./mcp-server-rust/target/release/mcp-server-rust /usr/local/bin/mcp-server

EXPOSE 8080

CMD ["mcp-server", "start", "--address", "0.0.0.0:8080"]
