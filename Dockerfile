ARG RUST_VERSION=1.74.0
ARG APP_NAME=rustacean

FROM rust:${RUST_VERSION}-slim-bullseye AS build
ARG APP_NAME
WORKDIR /app

RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=bind,source=data,target=data \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    <<EOF
set -e
apt -y update
apt -y install pkg-config
apt -y install libssl-dev
cargo build --locked --release
cp ./target/release/$APP_NAME /bin/server
EOF

FROM debian:bullseye-slim AS final

ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser
RUN apt -y update
RUN apt -y install ca-certificates
USER appuser

# Copy the executable from the "build" stage.
COPY --from=build /bin/server /bin/

# Expose the port that the application listens on.
EXPOSE 8443

# What the container should run when it is started.
CMD ["/bin/server"]