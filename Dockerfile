FROM rust:1.67 as builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian:buster-slim

RUN apt-get update && apt-get install -y build-essential libpq-dev && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/fofoprono-server /usr/local/bin

CMD ["fofoprono-server"]
