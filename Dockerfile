# Dockerfile
FROM rust:1.70 as builder

WORKDIR /usr/src/mcp-server
COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/mcp-server/target/release/mcp-server-rust /usr/local/bin/mcp-server

EXPOSE 8080

CMD ["mcp-server", "start", "--address", "0.0.0.0:8080"]
