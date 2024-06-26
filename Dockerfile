ARG RUST_VERSION=1.78.0
ARG APP_NAME=ericminassian
FROM rust:${RUST_VERSION}-slim-bookworm AS build
ARG APP_NAME
WORKDIR /app

# Install Node.js and npm
RUN apt-get update && apt-get install -y nodejs npm

RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=assets,target=assets \
    --mount=type=bind,source=templates,target=templates \
    --mount=type=bind,source=tailwind.config.js,target=tailwind.config.js \
    --mount=type=bind,source=build.rs,target=build.rs \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/app/target \
    --mount=type=cache,target=/usr/local/cargo/registry \
    <<EOF
set -e
cargo build --locked --release
cp ./target/release/${APP_NAME} /bin/server
EOF

FROM debian:bookworm-slim as final

ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser
USER appuser

COPY --from=build /bin/server /bin/

EXPOSE 3000

CMD [ "/bin/server" ]