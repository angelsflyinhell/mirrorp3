FROM rust:latest

WORKDIR /usr/src
COPY . .

RUN cargo install --path .
RUN cargo build --release

EXPOSE 80

CMD ["./target/release/mirrorp3"]