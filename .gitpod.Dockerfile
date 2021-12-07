FROM gitpod/workspace-full

USER gitpod

ENV CARGO_HOME=

RUN bash -cl "rustup update stable"
RUN bash -cl "rustup update nightly"

RUN bash -cl "cargo install sqlx-cli --no-default-features --features postgres"

RUN sudo apt-get update && \
    sudo apt-get install -y postgresql-client && \
    sudo rm -rf /var/lib/apt/lists/*
