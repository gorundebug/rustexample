#!/bin/sh
set -eu

root=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
tmp="${TMPDIR:-/tmp}/order_service_api-openapi"
rm -rf "$tmp"
openapi-generator generate -g rust-axum -i "$root/openapi/orderserviceapi/orderserviceapi.generated.yaml" -o "$tmp" \
  --additional-properties=packageName=order_service_api
mkdir -p "$root/src/generated/apis"
cp "$tmp/src/header.rs" "$root/src/generated/header.generated.rs"
cp "$tmp/src/models.rs" "$root/src/generated/models.generated.rs"
cp "$tmp/src/types.rs" "$root/src/generated/types.generated.rs"
cp "$tmp/src/apis/default.rs" "$root/src/generated/apis/default.generated.rs"
sed 's/^pub mod default;$/#[path = "default.generated.rs"]\
pub mod default;/' "$tmp/src/apis/mod.rs" > "$root/src/generated/apis/mod.generated.rs"
cp "$tmp/src/server/mod.rs" "$root/src/generated/server.generated.rs"
for generated in "$root/src/generated/apis/mod.generated.rs" "$root/src/generated/server.generated.rs"; do
  filtered="$generated.notrace"
  sed '/#\[tracing::instrument(skip_all)\]/d' "$generated" > "$filtered"
  mv "$filtered" "$generated"
done