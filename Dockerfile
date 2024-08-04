FROM rust:latest as builder

WORKDIR /app

RUN apt update && apt -y install musl-tools
RUN rustup target add x86_64-unknown-linux-musl

COPY Cargo.toml Cargo.lock Makefile ./
COPY assets assets
COPY src src
RUN make build BUILD_TYPE=release TARGET=x86_64-unknown-linux-musl

FROM scratch

COPY --from=builder /app/build /app

CMD ["/app/tic_tac_toe_api"]