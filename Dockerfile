# ---------- Build ----------
FROM rust:1.82-bookworm AS builder

WORKDIR /app
COPY Cargo.toml Cargo.lock* ./
# Pre-build deps (cache layer)
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release && rm -rf src

COPY src/ src/
COPY templates/ templates/
RUN touch src/main.rs && cargo build --release

# ---------- Runtime ----------
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates libgcc-s1 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/festival-sport .
COPY templates/ templates/
COPY static/ static/

ENV RUST_LOG=info
ENV PORT=10000
EXPOSE 10000

CMD ["./festival-sport"]
