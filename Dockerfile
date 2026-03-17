FROM node:22-bookworm-slim AS frontend-build
WORKDIR /workspace/frontend
COPY frontend/package.json frontend/package-lock.json ./
RUN npm ci
COPY frontend/ ./
RUN npm run build

FROM rust:1.86-bookworm AS backend-build
WORKDIR /workspace
COPY Cargo.toml Cargo.lock ./
COPY backend/Cargo.toml backend/Cargo.toml
COPY backend/src backend/src
COPY backend/tests backend/tests
COPY examples examples
RUN cargo build --release -p lanuminous

FROM debian:bookworm-slim AS runtime
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /opt/lanuminous
COPY --from=backend-build /workspace/target/release/lanuminous /usr/local/bin/lanuminous
COPY --from=frontend-build /workspace/frontend/dist /opt/lanuminous/ui

ENV LANUMINOUS_STATE_DIR=/var/lib/lanuminous
EXPOSE 9097
VOLUME ["/config", "/var/lib/lanuminous"]

ENTRYPOINT ["lanuminous"]
CMD ["serve", "--config", "/config", "--listen", "0.0.0.0:9097", "--ui-dir", "/opt/lanuminous/ui", "--state-dir", "/var/lib/lanuminous"]
