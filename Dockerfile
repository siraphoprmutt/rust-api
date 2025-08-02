# ใช้ nightly image ที่รองรับ edition 2024
FROM rustlang/rust:nightly-slim as builder

WORKDIR /app
COPY . .
RUN cargo build --release

# Runtime stage
FROM debian:bullseye-slim
WORKDIR /app
COPY --from=builder /app/target/release/your-app-name /usr/local/bin/app
COPY public ./public
COPY db ./db
EXPOSE 3000
CMD ["app"]
