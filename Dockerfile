FROM rust:1.68.2

WORKDIR /app
COPY . .

RUN rustup default nightly
RUN cargo build --release

CMD cargo run --release