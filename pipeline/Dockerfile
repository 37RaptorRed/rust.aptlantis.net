FROM rust:1-bookworm AS builder

WORKDIR /src
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim

RUN useradd --create-home --uid 10001 aptlantis
COPY --from=builder /src/target/release/pipeline /usr/local/bin/rust-aptlantis-pipeline

USER aptlantis
WORKDIR /srv/rust.aptlantis
ENV RUST_APTLANTIS_DATA_ROOT=/srv/rust.aptlantis/data

ENTRYPOINT ["/usr/local/bin/rust-aptlantis-pipeline"]
