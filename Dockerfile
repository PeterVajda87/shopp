FROM rust:latest AS builder

RUN cargo new shopp
WORKDIR /shopp
COPY ./Cargo.toml ./Cargo.toml
RUN touch ./src/lib.rs
RUN cargo build --release

RUN rm -rf ./src
COPY ./src ./src
COPY ./migrations ./migrations
COPY ./static ./static
COPY ./.sqlx ./.sqlx
RUN touch ./src/main.rs && touch ./src/lib.rs
ENV SQLX_OFFLINE true
  
RUN cargo build --release

FROM debian:bookworm as runtime
WORKDIR /shopp
RUN apt-get update && apt install -y openssl && rm -rf /var/lib/apt/lists/* && apt-get clean
COPY ./settings.yaml .
COPY ./migrations ./migrations
COPY ./static ./static
COPY --from=builder /shopp/target/release/shopp ./shopp

CMD ["./shopp"]