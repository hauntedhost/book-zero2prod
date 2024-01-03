FROM rust:1.75.0
WORKDIR /app
RUN apt update && apt install lld clang -y
COPY . .
RUN SQLX_OFFLINE=true cargo build --release
ENV APP_ENV prod
ENTRYPOINT ["./target/release/zero2prod"]
