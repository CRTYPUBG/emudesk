# ---- Build stage ----
FROM rust:1.93-slim AS builder
WORKDIR /build
COPY . .
# Build only the coordination server binary (release).
RUN cargo build --release -p emudesk-server

# ---- Runtime stage ----
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /build/target/release/emudesk-server /app/emudesk-server

# HTTP coordination port (also read from $PORT). Health is served on the same port.
EXPOSE 5876

# Non-root user.
RUN useradd --create-home --uid 10001 emudesk
USER emudesk

# Optional API token for registrations. Empty in dev.
ENV API_TOKEN=""

ENTRYPOINT ["/app/emudesk-server"]