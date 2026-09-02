ARG DEPENDENCY_DOCKER_REGISTRY=docker.io
FROM rustservicelib-source AS rustservicelib-source
FROM module-model_rust-source AS module-model_rust-source
FROM module-inventory_service_api-source AS module-inventory_service_api-source
FROM ${DEPENDENCY_DOCKER_REGISTRY}/library/rust:1.97-bookworm AS development

COPY dependency-download-mirrors.generated.env /etc/servicegen/dependency-download-mirrors.generated.env
COPY dependency-download-mirrors.env /etc/servicegen/dependency-download-mirrors.env
COPY dependency-download-env.generated.sh /usr/local/bin/dependency-download-env
SHELL ["/usr/local/bin/dependency-download-env", "/bin/sh", "-c"]
ARG OPENAPI_GENERATOR_VERSION=7.24.0
ARG CARGO_REGISTRIES_CRATES_IO_INDEX=https://github.com/rust-lang/crates.io-index
ARG DEPENDENCY_MAVEN_CENTRAL_URL=https://repo1.maven.org/maven2
ARG DEPENDENCY_APT_DEBIAN_URL=
ARG DEPENDENCY_APT_DEBIAN_SECURITY_URL=
RUN if [ -n "$DEPENDENCY_APT_DEBIAN_URL$DEPENDENCY_APT_DEBIAN_SECURITY_URL" ]; then \
      find /etc/apt -type f \( -name '*.list' -o -name '*.sources' \) -exec sed -i \
        -e "s|http://deb.debian.org/debian-security|$DEPENDENCY_APT_DEBIAN_SECURITY_URL|g" \
        -e "s|http://deb.debian.org/debian|$DEPENDENCY_APT_DEBIAN_URL|g" {} +; \
    fi
RUN apt-get update && apt-get install --yes --no-install-recommends \
      ca-certificates cmake curl default-jre-headless gdbserver \
    && curl --fail --location --silent --show-error --connect-timeout 15 --speed-limit 1024 --speed-time 60 --retry 8 --retry-delay 2 --retry-max-time 600 --retry-all-errors \
      "${DEPENDENCY_MAVEN_CENTRAL_URL}/org/openapitools/openapi-generator-cli/${OPENAPI_GENERATOR_VERSION}/openapi-generator-cli-${OPENAPI_GENERATOR_VERSION}.jar" \
      --output /opt/openapi-generator-cli.jar \
    && printf '%s\n' '#!/bin/sh' 'exec java -jar /opt/openapi-generator-cli.jar "$@"' > /usr/local/bin/openapi-generator \
    && chmod 0755 /usr/local/bin/openapi-generator \
    && rm -rf /var/lib/apt/lists/*
RUN if [ "$CARGO_REGISTRIES_CRATES_IO_INDEX" != "https://github.com/rust-lang/crates.io-index" ]; then \
      mkdir -p /usr/local/cargo; \
      printf '%s\n' '[source.crates-io]' 'replace-with = "dependency-proxy"' \
        '[source.dependency-proxy]' "registry = \"$CARGO_REGISTRIES_CRATES_IO_INDEX\"" \
        > /usr/local/cargo/config.toml; \
    fi
ENV CARGO_HTTP_MULTIPLEXING=false CARGO_HTTP_TIMEOUT=30 CARGO_NET_RETRY=5
COPY prepare-local-build.generated.sh /usr/local/bin/prepare-local-build
WORKDIR /workspace
COPY --from=rustservicelib-source / /tmp/rustservicelib-source
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
    mkdir -p /workspace/rustservicelib; \
    cp -R "$source_dir/." /workspace/rustservicelib/; \
    rm -rf /tmp/rustservicelib-source /tmp/rustservicelib-archive
COPY --from=module-model_rust-source / /tmp/module-model_rust-source
RUN set -eu; \
    source_dir=/tmp/module-model_rust-source; \
    archive=$(find "$source_dir" -mindepth 1 -maxdepth 1 -type f \( -name context -o -name '*.tar' -o -name '*.tar.gz' -o -name '*.tgz' -o -name '*.tar.xz' \) -print -quit); \
    if [ -n "$archive" ]; then \
      mkdir -p /tmp/module-model_rust-archive; \
      tar -xf "$archive" -C /tmp/module-model_rust-archive; \
      source_dir=/tmp/module-model_rust-archive; \
    fi; \
    manifest=$(find "$source_dir" -type f -path '*/model_rust/Cargo.toml' -print -quit); \
    if [ -z "$manifest" ]; then manifest="$source_dir/Cargo.toml"; fi; \
    if [ ! -f "$manifest" ]; then manifest=$(find "$source_dir" -mindepth 2 -maxdepth 2 -type f -name Cargo.toml -print -quit); fi; \
    if [ -z "$manifest" ] || [ ! -f "$manifest" ]; then echo "module model_rust source context has no Cargo.toml" >&2; exit 1; fi; \
    source_dir=${manifest%/Cargo.toml}; \
    mkdir -p /workspace/modules/model_rust; \
    cp -R "$source_dir/." /workspace/modules/model_rust/; \
    rm -rf /tmp/module-model_rust-source /tmp/module-model_rust-archive
COPY --from=module-inventory_service_api-source / /tmp/module-inventory_service_api-source
RUN set -eu; \
    source_dir=/tmp/module-inventory_service_api-source; \
    archive=$(find "$source_dir" -mindepth 1 -maxdepth 1 -type f \( -name context -o -name '*.tar' -o -name '*.tar.gz' -o -name '*.tgz' -o -name '*.tar.xz' \) -print -quit); \
    if [ -n "$archive" ]; then \
      mkdir -p /tmp/module-inventory_service_api-archive; \
      tar -xf "$archive" -C /tmp/module-inventory_service_api-archive; \
      source_dir=/tmp/module-inventory_service_api-archive; \
    fi; \
    manifest=$(find "$source_dir" -type f -path '*/inventory_service_api/Cargo.toml' -print -quit); \
    if [ -z "$manifest" ]; then manifest="$source_dir/Cargo.toml"; fi; \
    if [ ! -f "$manifest" ]; then manifest=$(find "$source_dir" -mindepth 2 -maxdepth 2 -type f -name Cargo.toml -print -quit); fi; \
    if [ -z "$manifest" ] || [ ! -f "$manifest" ]; then echo "module inventory_service_api source context has no Cargo.toml" >&2; exit 1; fi; \
    source_dir=${manifest%/Cargo.toml}; \
    mkdir -p /workspace/modules/inventory_service_api; \
    cp -R "$source_dir/." /workspace/modules/inventory_service_api/; \
    rm -rf /tmp/module-inventory_service_api-source /tmp/module-inventory_service_api-archive

# Dev/debug uses the same dependency contexts; only the service source is mounted.
WORKDIR /workspace/service

FROM development AS build
COPY . /workspace/service
RUN set -eu; \
    sed -i -E 's|^servicelib-gorundebug[[:space:]]*=.*$|servicelib-gorundebug = { path = "/workspace/rustservicelib" }|' Cargo.toml; \
    grep -F 'servicelib-gorundebug = { path = "/workspace/rustservicelib" }' Cargo.toml >/dev/null
RUN set -eu; \
    sed -i -E 's|^example-model[[:space:]]*=.*$|example-model = { path = "/workspace/modules/model_rust" }|' Cargo.toml; \
    grep -F 'example-model = { path = "/workspace/modules/model_rust" }' Cargo.toml >/dev/null
RUN set -eu; \
    sed -i -E 's|^inventory-service-api[[:space:]]*=.*$|inventory-service-api = { path = "/workspace/modules/inventory_service_api" }|' Cargo.toml; \
    grep -F 'inventory-service-api = { path = "/workspace/modules/inventory_service_api" }' Cargo.toml >/dev/null
RUN find /workspace/modules -name 'generate-openapi.generated.sh' -exec {} \;
RUN --mount=type=cache,id=inventoryservice-cargo-registry,target=/usr/local/cargo/registry \
    --mount=type=cache,id=inventoryservice-cargo-git,target=/usr/local/cargo/git \
    --mount=type=cache,id=inventoryservice-target,target=/workspace/service/target \
    service_binary="$(sed -n '/^\[package\]/,/^\[/s/^name = "\([^"]*\)"/\1/p' Cargo.toml | head -n 1)" \
    && test -n "$service_binary" \
    && cargo build --release \
    && cp "target/release/$service_binary" /workspace/service-binary

FROM ${DEPENDENCY_DOCKER_REGISTRY}/library/debian:bookworm-slim AS runtime
COPY dependency-download-mirrors.generated.env /etc/servicegen/dependency-download-mirrors.generated.env
COPY dependency-download-mirrors.env /etc/servicegen/dependency-download-mirrors.env
COPY dependency-download-env.generated.sh /usr/local/bin/dependency-download-env
SHELL ["/usr/local/bin/dependency-download-env", "/bin/sh", "-c"]
ARG DEPENDENCY_APT_DEBIAN_URL=
ARG DEPENDENCY_APT_DEBIAN_SECURITY_URL=
RUN if [ -n "$DEPENDENCY_APT_DEBIAN_URL$DEPENDENCY_APT_DEBIAN_SECURITY_URL" ]; then \
      find /etc/apt -type f \( -name '*.list' -o -name '*.sources' \) -exec sed -i \
        -e "s|http://deb.debian.org/debian-security|$DEPENDENCY_APT_DEBIAN_SECURITY_URL|g" \
        -e "s|http://deb.debian.org/debian|$DEPENDENCY_APT_DEBIAN_URL|g" {} +; \
    fi
RUN apt-get update && apt-get install --yes --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=build /workspace/service-binary /usr/local/bin/service
COPY config /app/config
ENTRYPOINT ["/usr/local/bin/service", "--config", "/app/config/config.yaml", "--values", "/app/config/overrides.yaml"]