FROM lukemathwalker/cargo-chef:latest AS chef
WORKDIR /app

FROM chef AS planner
COPY ./Cargo.toml ./Cargo.lock ./
COPY ./src ./src
RUN cargo chef prepare

FROM chef AS builder
COPY --from=planner /app/recipe.json .
RUN cargo chef cook --release
COPY . .
RUN cargo build --release
RUN mv ./target/release/spotoxide ./app

# debian as we need libssl
FROM debian:stable-slim AS runtime
WORKDIR /app
RUN apt update
RUN apt install -y libssl-dev
COPY --from=builder /app/app /usr/local/bin/
EXPOSE 3000
ENTRYPOINT ["/usr/local/bin/app"]
