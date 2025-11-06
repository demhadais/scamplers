# syntax=docker/dockerfile:1

ARG RUST_VERSION=1

FROM rust:${RUST_VERSION}-bookworm AS build
WORKDIR /app

RUN apt update && \
    apt install make openssl --yes && \
    cargo install diesel_cli --no-default-features --features postgres-bundled



RUN --mount=type=bind,source=default-vec/Cargo.toml,target=default-vec/Cargo.toml \
    --mount=type=bind,source=rust/Cargo.lock,target=rust/Cargo.lock \
    --mount=type=bind,source=rust/.cargo,target=rust/.cargo \
    --mount=type=bind,source=rust/any-value,target=rust/any-value \
    --mount=type=bind,source=rust/scamplepy,target=rust/scamplepy \
    --mount=type=bind,source=rust/scamplers,target=rust/scamplers \
    --mount=type=bind,source=rust/scamplers-macros,target=rust/scamplers-macros \
    --mount=type=bind,source=rust/scamplers-schema,target=rust/scamplers-schema \
    --mount=type=bind,source=rust/time,target=rust/time \
    --mount=type=bind,source=rust/uuid,target=rust/uuid \
    --mount=type=bind,source=rust/valid-string,target=rust/valid-string \
    --mount=type=bind,source=db,target=db \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    cd rust && \
    IN_DOCKER=true cargo build --locked --release --features app && \
    cp ./target/release/scamplers /bin/scamplers

FROM debian:bookworm AS final

RUN apt update && apt install curl libpq-dev --yes

RUN mkdir app

COPY --from=build /bin/scamplers /bin/
COPY --from=build /usr/local/cargo/bin/diesel /bin/
COPY rust/scamplers-schema /app/scamplers-schema
COPY db /app/db
COPY rust/check-schema.sh /bin/check-schema.sh

ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser
RUN mkdir logs && chown appuser logs
USER appuser

EXPOSE ${PORT}

CMD /bin/check-schema.sh && /bin/scamplers --log-dir logs --host 0.0.0.0 --port $PORT --secrets-dir /run/secrets --db-host $DB_HOST --db-port $DB_PORT
