FROM rust:bookworm AS builder
WORKDIR /app
COPY . .
RUN cargo run -p generator --release && cargo build -p demo_app --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/demo_app /app/demo_app
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
CMD ["./demo_app"]
