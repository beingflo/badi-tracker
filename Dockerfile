FROM rust:1.87 AS chef 
RUN cargo install cargo-chef 
WORKDIR /usr/src/badi-tracker

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /usr/src/badi-tracker/recipe.json recipe.json

RUN cargo chef cook --release --recipe-path recipe.json
RUN cargo build --release --bin badi-tracker 

FROM debian:bookworm-slim AS runtime

WORKDIR /usr/src/app/
COPY --from=builder /usr/src/badi-tracker/target/release/badi-tracker /usr/src/app/
ENTRYPOINT ["/usr/src/app/badi-tracker"]