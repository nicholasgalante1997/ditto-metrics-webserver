FROM rust:1.69-slim-buster

RUN mkdir -p /app/services/ditto

WORKDIR /app/services/ditto

COPY . .

RUN cargo build --release 

RUN rm -rf src/

EXPOSE 7100

CMD ["./target/release/ditto-server"]

