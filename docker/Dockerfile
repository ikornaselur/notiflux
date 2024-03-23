FROM rust:1.77-buster AS builder

WORKDIR /usr/src
RUN apt-get update -y && apt-get install musl-tools ca-certificates -y
RUN rustup target add x86_64-unknown-linux-musl

RUN USER=root cargo new notiflux
WORKDIR /usr/src/notiflux

COPY Cargo.toml Cargo.lock ./
RUN cargo build --release --target x86_64-unknown-linux-musl --bin notiflux

COPY src ./src
RUN cargo build --release --target x86_64-unknown-linux-musl --bin notiflux

FROM scratch

COPY --from=builder /usr/src/notiflux/target/x86_64-unknown-linux-musl/release/notiflux .
COPY --from=builder /etc/ssl /etc/ssl
USER 1000
EXPOSE 8080

CMD ["./notiflux"]

