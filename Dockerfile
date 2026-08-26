FROM rustservicelib-source AS rustservicelib-source
FROM rust:1.97-bookworm AS build

ARG OPENAPI_GENERATOR_VERSION=7.24.0
ARG CARGO_REGISTRIES_CRATES_IO_INDEX=https://github.com/rust-lang/crates.io-index
ARG SERVICEGEN_MAVEN_CENTRAL_URL=https://repo1.maven.org/maven2
ARG SERVICEGEN_APT_DEBIAN_URL=
ARG SERVICEGEN_APT_DEBIAN_SECURITY_URL=
RUN if [ -n "$SERVICEGEN_APT_DEBIAN_URL$SERVICEGEN_APT_DEBIAN_SECURITY_URL" ]; then \
      find /etc/apt -type f \( -name '*.list' -o -name '*.sources' \) -exec sed -i \
        -e "s|http://deb.debian.org/debian-security|$SERVICEGEN_APT_DEBIAN_SECURITY_URL|g" \
        -e "s|http://deb.debian.org/debian|$SERVICEGEN_APT_DEBIAN_URL|g" {} +; \
    fi
RUN apt-get update \
    && apt-get install --yes --no-install-recommends \
       ca-certificates cmake curl default-jre-headless \
    && curl --fail --location --show-error \
       "${SERVICEGEN_MAVEN_CENTRAL_URL}/org/openapitools/openapi-generator-cli/${OPENAPI_GENERATOR_VERSION}/openapi-generator-cli-${OPENAPI_GENERATOR_VERSION}.jar" \
       --output /opt/openapi-generator-cli.jar \
    && printf '%s\n' '#!/bin/sh' 'exec java -jar /opt/openapi-generator-cli.jar "$@"' \
       > /usr/local/bin/openapi-generator \
    && chmod 0755 /usr/local/bin/openapi-generator \
    && rm -rf /var/lib/apt/lists/*

# Cargo's crates.io source is special: changing only
# CARGO_REGISTRIES_CRATES_IO_INDEX does not reliably redirect crate downloads.
# Install an explicit source replacement only when the opt-in proxy URL was
# supplied; ordinary builds retain Cargo's default crates.io behaviour.
RUN if [ "$CARGO_REGISTRIES_CRATES_IO_INDEX" != "https://github.com/rust-lang/crates.io-index" ]; then \
      mkdir -p /usr/local/cargo; \
      printf '%s\n' \
        '[source.crates-io]' \
        'replace-with = "servicegen"' \
        '[source.servicegen]' \
        "registry = \"$CARGO_REGISTRIES_CRATES_IO_INDEX\"" \
        > /usr/local/cargo/config.toml; \
    fi

ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse \
    CARGO_REGISTRIES_CRATES_IO_INDEX=${CARGO_REGISTRIES_CRATES_IO_INDEX} \
    CARGO_HTTP_MULTIPLEXING=false \
    CARGO_HTTP_TIMEOUT=30 \
    CARGO_NET_RETRY=5

WORKDIR /workspace
COPY . /workspace/rustexample
COPY --from=rustservicelib-source . /tmp/rustservicelib-source
RUN set -eu; \
    source_dir=/tmp/rustservicelib-source; \
    archive=$(find "$source_dir" -mindepth 1 -maxdepth 1 -type f \( -name context -o -name '*.tar' -o -name '*.tar.gz' -o -name '*.tgz' -o -name '*.tar.xz' \) -print -quit); \
    if [ -n "$archive" ]; then \
      mkdir -p /tmp/rustservicelib-archive; \
      tar -xf "$archive" -C /tmp/rustservicelib-archive; \
      source_dir=/tmp/rustservicelib-archive; \
    fi; \
    manifest="$source_dir/Cargo.toml"; \
    if [ ! -f "$manifest" ]; then manifest=$(find "$source_dir" -mindepth 2 -maxdepth 2 -type f -name Cargo.toml -print -quit); fi; \
    if [ -z "$manifest" ]; then echo "rustservicelib source context has no Cargo.toml" >&2; exit 1; fi; \
    source_dir=${manifest%/Cargo.toml}; \
    if [ -z "$source_dir" ] || [ "$source_dir" = "/" ]; then echo "unsafe rustservicelib source directory" >&2; exit 1; fi; \
    mkdir -p /workspace/rustservicelib; \
    cp -a "$source_dir/." /workspace/rustservicelib/; \
    rm -rf /tmp/rustservicelib-source
WORKDIR /workspace/rustexample
# HTTP bindings are language-tool outputs, so a clean Docker build must create
# them before Cargo resolves the workspace. The pinned CLI download is cached
# in the immutable tool layer and is not repeated for source-only changes.
RUN find . -name 'generate-openapi.generated.sh' -exec {} \;
# Generated archives intentionally use deterministic timestamps. Refresh the
# workspace mtimes so Cargo's persistent target cache cannot reuse a service
# binary after a same-size generated source change.
RUN find . -type f -not -path './target/*' -exec touch {} +
RUN --mount=type=cache,id=rustexample-cargo-registry,target=/usr/local/cargo/registry \
    --mount=type=cache,id=rustexample-cargo-git,target=/usr/local/cargo/git \
    --mount=type=cache,id=rustexample-target,target=/workspace/rustexample/target \
    true \
    && analyticsservice_binary="$(sed -n '/^\[package\]/,/^\[/s/^name = "\([^"]*\)"/\1/p' analyticsservice/Cargo.toml | head -n 1)" \
    && test -n "$analyticsservice_binary" \
    && inventoryservice_binary="$(sed -n '/^\[package\]/,/^\[/s/^name = "\([^"]*\)"/\1/p' inventoryservice/Cargo.toml | head -n 1)" \
    && test -n "$inventoryservice_binary" \
    && orderservice_binary="$(sed -n '/^\[package\]/,/^\[/s/^name = "\([^"]*\)"/\1/p' orderservice/Cargo.toml | head -n 1)" \
    && test -n "$orderservice_binary" \
    && cargo --config 'patch."https://github.com/gorundebug/rustservicelib.git".servicelib-gorundebug.path="/workspace/rustservicelib"' \
        build --locked --release --workspace --bins \
    && cp "target/release/$analyticsservice_binary" /workspace/analyticsservice \
    && cp "target/release/$inventoryservice_binary" /workspace/inventoryservice \
    && cp "target/release/$orderservice_binary" /workspace/orderservice \
    && true

FROM debian:bookworm-slim AS runtime
RUN apt-get update \
    && apt-get install --yes --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app

FROM runtime AS analyticsservice
COPY --from=build /workspace/analyticsservice /usr/local/bin/service
COPY analyticsservice/config /app/config
EXPOSE 9093 9203
ENTRYPOINT ["/usr/local/bin/service"]

FROM runtime AS inventoryservice
COPY --from=build /workspace/inventoryservice /usr/local/bin/service
COPY inventoryservice/config /app/config
EXPOSE 9092 9202
ENTRYPOINT ["/usr/local/bin/service"]

FROM runtime AS orderservice
COPY --from=build /workspace/orderservice /usr/local/bin/service
COPY orderservice/config /app/config
EXPOSE 9091 9201
ENTRYPOINT ["/usr/local/bin/service"]
