FROM rust:1.51

WORKDIR /usr/src/rust-app
COPY . .

RUN cargo install --path .

CMD ["rust-app"]