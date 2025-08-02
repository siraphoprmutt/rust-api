# 🛠 Build stage
FROM rust:slim as builder
WORKDIR /app
COPY . .
RUN apt-get update && apt-get upgrade -y && rm -rf /var/lib/apt/lists/* && \
    cargo build --release

# 🧼 Runtime stage: ใช้ image ที่เล็กมาก
FROM debian:bookworm-slim
WORKDIR /app

# Update and install security patches and remove unnecessary packages
RUN apt-get update && \
    apt-get upgrade -y && \
    apt-get dist-upgrade -y && \
    apt-get autoremove -y && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/rust-microservice /usr/local/bin/app
COPY public ./public
EXPOSE 3000
CMD ["app"]