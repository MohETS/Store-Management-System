FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /rocket_api

FROM chef AS planner
COPY ../.. .
RUN cargo chef prepare

FROM chef AS builder
RUN apt-get update && apt-get install -y libpq-dev libssl-dev pkg-config build-essential
COPY --from=planner /rocket_api/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY ../.. .
RUN cargo build --release --bin rocket_server


FROM ubuntu:latest AS runtime
RUN apt-get update && apt-get install -y libpq5 libssl3
WORKDIR /rocket_api
COPY --from=builder /rocket_api/target/release/rocket_server /usr/local/bin
ENTRYPOINT ["/usr/local/bin/rocket_server"]