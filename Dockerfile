# Builder stage
FROM rust:1.75.0 AS builder
WORKDIR /app
RUN apt update && apt install lld clang -y
COPY . .
RUN SQLX_OFFLINE=true cargo build --release

# Runtime stage
FROM debian:bookworm-slim AS runtime
WORKDIR /app
RUN apt-get update -y && \
    apt-get install -y --no-install-recommends \
      ca-certificates \
      openssl \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/zero2prod zero2prod
COPY config config
ENV APP_ENV prod
ENTRYPOINT ["./zero2prod"]
