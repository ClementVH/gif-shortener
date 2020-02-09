FROM rust

WORKDIR /usr/src/gif-shortener

RUN useradd rust --uid 1000

USER rust

RUN cargo install cargo-watch

