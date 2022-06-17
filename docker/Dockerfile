FROM lukemathwalker/cargo-chef:latest-rust-1.61.0 as chef
WORKDIR /app

FROM chef as planner
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
# Build our project dependencies, not our application!
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
ENV SQLX_OFFLINE true
# Build our project
RUN cargo build --release --bin conduit

FROM debian:11-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/conduit conduit
COPY configuration configuration
ENV APP_ENVIRONMENT production
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates curl \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

HEALTHCHECK --interval=5s --timeout=3s --retries=3 \
    CMD curl -f 127.0.0.1:8080/api/health_check

ENTRYPOINT ["./conduit"]
