FROM rust:1.85-slim-bookworm AS builder

WORKDIR /usr/src/app

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

COPY . .
RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && apt-get install -y \
    tesseract-ocr \
    tesseract-ocr-fra \
    tesseract-ocr-ara \
    tesseract-ocr-eng \
    poppler-utils \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

RUN useradd -ms /bin/bash botuser

COPY --from=builder /usr/src/app/target/release/telegram-ai-analyzer .

RUN mkdir temp && chown botuser:botuser temp

USER botuser

ENV RUST_LOG=info

CMD ["./telegram-ai-analyzer"]
