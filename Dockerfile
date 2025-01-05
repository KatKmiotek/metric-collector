FROM rust:1.83-slim

WORKDIR /app

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./

COPY . .

RUN cargo build --release

EXPOSE 8080

CMD ["/app/target/release/metric-collector"]
