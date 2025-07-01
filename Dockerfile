FROM rust:1.73 as builder
WORKDIR /app
COPY . .
RUN apt-get update && apt-get install -y pkg-config libsqlite3-dev && \
    cargo install --path .

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates sqlite3
COPY --from=builder /usr/local/cargo/bin/dashboard_api /usr/local/bin/dashboard_api
COPY ./migrations ./migrations
WORKDIR /app
EXPOSE 8080
CMD ["dashboard_api"]
