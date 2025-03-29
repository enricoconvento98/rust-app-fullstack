# Build stage
FROM rust:1.75 as builder

WORKDIR /usr/src/app
COPY . .

RUN cargo install --path .

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/backend /usr/local/bin/backend

ENV RUST_LOG=info
ENV PORT=3000

EXPOSE 3000

CMD ["backend"]