FROM rust:alpine as builder
RUN apk add --no-cache musl-dev
WORKDIR /usr/src/dollpublish
COPY . .
RUN mkdir /build && cargo install --path . --root /build

FROM alpine
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
WORKDIR /usr/local/bin
COPY --from=builder /build/bin/dollpublish .

VOLUME ["/data"]
EXPOSE 3000
CMD ["./dollpublish"]
