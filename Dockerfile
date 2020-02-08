FROM rust:1.41-alpine3.11 as builder

WORKDIR /work
COPY ./guessing_game .
RUN cargo build --release

FROM alpine:3.11

WORKDIR /app
COPY --from=builder /work/target/release/guessing_game .
CMD ["/app/guessing_game"]
