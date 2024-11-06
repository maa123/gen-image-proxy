FROM rust:1-alpine as builder

RUN apk add --no-cache musl-dev

WORKDIR /src/app

COPY . .

RUN cargo build --release

RUN strip target/release/gen-image-proxy

FROM gcr.io/distroless/static-debian12

COPY --from=builder /src/app/target/release/gen-image-proxy /gen-image-proxy

CMD ["/gen-image-proxy"]
