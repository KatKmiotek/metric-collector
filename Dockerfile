FROM rust:1.81-slim

WORKDIR /app

COPY . .

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

RUN cargo build --release
EXPOSE 8080


CMD ["./target/release/metric-collector"]
