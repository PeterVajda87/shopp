FROM rust:latest AS builder
WORKDIR /app

COPY Cargo.toml .
RUN mkdir src && echo "fn main()" > src/main.rs
RUN cargo build --release

COPY src src
RUN touch src/main.rs
RUN cargo build --release

RUN strip target/release/shopp

FROM gcr.io/distroless/static-debian12 as release
WORKDIR /app
COPY --from=builder /app/target/release/shopp .


ENV SQLX_OFFLINE true
CMD ["./shopp"]
