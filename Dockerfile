FROM rust:1.69 as builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

COPY src ./src

RUN cargo build --release

FROM debian:buster-slim

WORKDIR /app

COPY --from=builder /app/target/release/your_app_name .

EXPOSE 2805

CMD ["./simple-backend-api"]