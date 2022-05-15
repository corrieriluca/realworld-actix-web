FROM rust:1.60 as builder
ENV CARGO_INSTALL_ROOT /usr
RUN cargo install sqlx-cli --no-default-features --features rustls,postgres --version 0.5.13 --locked

FROM debian:11-slim AS runtime
COPY --from=builder /usr/bin/sqlx /usr/bin/sqlx
COPY docker/migrations.sh /usr/bin/migrations.sh
COPY migrations migrations
COPY sqlx-data.json sqlx-data.json
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends postgresql-client \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
ENTRYPOINT [ "migrations.sh" ]
