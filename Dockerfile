FROM rust:bookworm AS chef 
RUN cargo install cargo-chef 
WORKDIR /app

FROM chef AS planner
COPY ./ .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY ./ .
RUN cargo build --release --bin badi-tracker

FROM debian:bookworm-slim AS runtime
RUN apt update && apt install -y ca-certificates && apt install -y openssl
WORKDIR /app
COPY --from=builder /app/target/release/badi-tracker /usr/local/bin
ENTRYPOINT ["/usr/local/bin/badi-tracker"]