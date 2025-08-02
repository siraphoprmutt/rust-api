# 🛠 Build stage
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

# 🧼 Runtime stage: ใช้ image ที่เล็กมาก
FROM debian:bullseye-slim
WORKDIR /app
COPY --from=builder /app/target/release/rust-microservice /usr/local/bin/app
COPY public ./public
EXPOSE 3000
CMD ["app"]
