FROM gitpod/workspace-full

USER gitpod

RUN bash -cl "rustup toolchain install nightly"
RUN bash -cl "rustup toolchain install stable"

RUN bash -cl "cargo install sqlx-cli --no-default-features --features postgres"

RUN sudo apt-get update && \
    sudo apt-get install -y postgresql-client && \
    sudo rm -rf /var/lib/apt/lists/*
