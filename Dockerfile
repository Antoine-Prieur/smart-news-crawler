FROM rust:1.87 as builder

WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app
COPY --from=builder /app/target/release/smart-news-crawler ./app

CMD ["./app"]
