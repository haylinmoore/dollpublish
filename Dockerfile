FROM lukemathwalker/cargo-chef:0.1.67-rust-alpine3.19 AS planner
WORKDIR /app
COPY src/ src/
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
RUN cargo chef prepare --recipe-path recipe.json

FROM lukemathwalker/cargo-chef:0.1.67-rust-alpine3.19 AS builder
WORKDIR /app
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
COPY src/ src/
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
RUN --mount=type=cache,target=/usr/local/cargo/registry --mount=type=cache,target=/src/target cargo build --release --target x86_64-unknown-linux-musl 

FROM alpine AS runtime

ARG REF=""
ARG COMMIT=""
ARG TIME=""

ENV COMMIT=${COMMIT}
ENV REF=${REF}
ENV TIME=${TIME}
ENV TZ="America/New_York"
ENV MOON_DATA_DIR=/data
ENV MOON_BIND_ADDR=0.0.0.0
ENV MOON_PORT=3000

RUN mkdir -p /data

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/dollpublish /usr/local/bin/

WORKDIR /usr/local/bin
VOLUME ["/data"]
CMD ["/usr/local/bin/dollpublish"]
EXPOSE 3000
