FROM rust:1.70.0-slim-bookworm as build

WORKDIR /app

RUN apt-get update -qq && \
    DEBIAN_FRONTEND=noninteractive apt-get install pkg-config libssl-dev curl musl-tools musl-dev build-essential -y --no-install-recommends && \
    apt-get clean && find /var/lib/apt/lists -type f -delete

RUN rustup target add x86_64-unknown-linux-musl
ENV RUST_BACKTRACE=1

RUN cargo new --bin common
COPY common/Cargo.* common/

RUN echo "[workspace]\n"\
  "members = ['common']"\
  > Cargo.toml

COPY Cargo.lock .

RUN cargo build --release --target x86_64-unknown-linux-musl

RUN cargo new --bin worker
COPY worker/Cargo.* worker/

RUN echo "[workspace]\n"\
  "members = ['common', 'worker']"\
  > Cargo.toml

RUN cargo build --release --target x86_64-unknown-linux-musl

COPY common/src common/src
COPY worker/src worker/src

RUN cargo build --release --target x86_64-unknown-linux-musl

FROM scratch

WORKDIR /worker
COPY --from=build /app/target/x86_64-unknown-linux-musl/release/worker .
ENV RUST_LOG=debug
ENV RUST_BACKTRACE=full
EXPOSE 8000

ENTRYPOINT [ "./worker"]
CMD [ "./worker"]
