FROM rust:1.67 as builder
RUN USER=root cargo new --bin myapp
WORKDIR myapp
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src
RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update && apt-get install && rm -rf /var/lib/apt/lists/*
COPY --from=builder /myapp/target/release/among-us-server /usr/local/bin/among-us-server
EXPOSE 9090
CMD ["among-us-server"]
