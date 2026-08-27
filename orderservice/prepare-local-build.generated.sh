#!/bin/sh
set -eu

source_dir="${1:?source directory is required}"
work_dir="${2:?work directory is required}"

test -f "$source_dir/Cargo.toml"
mkdir -p "$work_dir"
find "$work_dir" -mindepth 1 -maxdepth 1 ! -name target -exec rm -rf -- {} +
cp -a "$source_dir/." "$work_dir/"

manifest="$work_dir/Cargo.toml"
sed -i -E 's|^servicelib-gorundebug[[:space:]]*=.*$|servicelib-gorundebug = { path = "/workspace/rustservicelib" }|' "$manifest"
grep -F 'servicelib-gorundebug = { path = "/workspace/rustservicelib" }' "$manifest" >/dev/null
sed -i -E 's|^example-model[[:space:]]*=.*$|example-model = { path = "/workspace/modules/model" }|' "$manifest"
grep -F 'example-model = { path = "/workspace/modules/model" }' "$manifest" >/dev/null
sed -i -E 's|^order-service-api[[:space:]]*=.*$|order-service-api = { path = "/workspace/modules/order_service_api" }|' "$manifest"
grep -F 'order-service-api = { path = "/workspace/modules/order_service_api" }' "$manifest" >/dev/null
sed -i -E 's|^inventory-service-api[[:space:]]*=.*$|inventory-service-api = { path = "/workspace/modules/inventory_service_api" }|' "$manifest"
grep -F 'inventory-service-api = { path = "/workspace/modules/inventory_service_api" }' "$manifest" >/dev/null