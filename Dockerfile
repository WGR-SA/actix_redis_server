FROM rust:1.79-slim-buster as builder

WORKDIR /usr/src/app
COPY . .

RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/actix_redis_server /usr/local/bin/actix_redis_server

# Copy default .env file
COPY .env /usr/local/bin/.env

# Set environment variables to be used by default, they can be overridden by Jelastic environment variables
ENV REDIS_URL=redis://host.docker.internal/
ENV REDIS_PREFIX=myapp:
ENV PORT=80
ENV RUST_LOG=info

CMD ["actix_redis_server"]
