FROM rust:1.71-slim
WORKDIR /app
COPY . .
RUN cargo build 
ENTRYPOINT ["/app/target/debug/http_echo"]