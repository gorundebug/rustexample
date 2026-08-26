#!/usr/bin/env bash
set -euo pipefail

if (($# != 2)); then
  echo "usage: $0 <service-directory> <output-directory>" >&2
  exit 2
fi

service_dir="${1%/}"
output_dir="${2%/}"
service_name="$(basename "${service_dir}")"

for file in Cargo.toml Makefile \
  make.generated.mk .gitignore README.md; do
  if [[ ! -f "${service_dir}/${file}" ]]; then
    echo "Rust service publishing file is missing: ${service_dir}/${file}" >&2
    exit 1
  fi
done
if [[ -e "${output_dir}" &&
      -n "$(find "${output_dir}" -mindepth 1 -maxdepth 1 -print -quit)" ]]; then
  echo "output directory must be empty: ${output_dir}" >&2
  exit 1
fi

mkdir -p "${output_dir}"
cp -R "${service_dir}/." "${output_dir}/"
rm -f "${output_dir}/Cargo.lock"

# Preserve the complete user-owned manifest, including dependencies added by
# business code. Only repository-boundary path dependencies are rewritten.
case "${service_name}" in
  "analyticsservice")
    sed -i.bak \
      's|^example-model = { path = "../model" }$|example-model = { git = "https://github.com/gorundebug/rustexample-model.git", tag = "v0.2.13" }|' \
      "${output_dir}/Cargo.toml"
    rm -f "${output_dir}/Cargo.toml.bak"
    ;;
  "inventoryservice")
    sed -i.bak \
      's|^example-model = { path = "../model" }$|example-model = { git = "https://github.com/gorundebug/rustexample-model.git", tag = "v0.2.13" }|' \
      "${output_dir}/Cargo.toml"
    rm -f "${output_dir}/Cargo.toml.bak"
    sed -i.bak \
      's|^inventory-service-api = { path = "../inventory_service_api" }$|inventory-service-api = { git = "https://github.com/gorundebug/rustexample-inventory-service-api.git", tag = "v0.2.13" }|' \
      "${output_dir}/Cargo.toml"
    rm -f "${output_dir}/Cargo.toml.bak"
    ;;
  "orderservice")
    sed -i.bak \
      's|^example-model = { path = "../model" }$|example-model = { git = "https://github.com/gorundebug/rustexample-model.git", tag = "v0.2.13" }|' \
      "${output_dir}/Cargo.toml"
    rm -f "${output_dir}/Cargo.toml.bak"
    sed -i.bak \
      's|^order-service-api = { path = "../order_service_api" }$|order-service-api = { git = "https://github.com/gorundebug/rustexample-order-service-api.git", tag = "v0.2.13" }|' \
      "${output_dir}/Cargo.toml"
    rm -f "${output_dir}/Cargo.toml.bak"
    sed -i.bak \
      's|^inventory-service-api = { path = "../inventory_service_api" }$|inventory-service-api = { git = "https://github.com/gorundebug/rustexample-inventory-service-api.git", tag = "v0.2.13" }|' \
      "${output_dir}/Cargo.toml"
    rm -f "${output_dir}/Cargo.toml.bak"
    ;;
  *)
    echo "unknown generated Rust service: ${service_name}" >&2
    exit 1
    ;;
esac

echo "Packaged standalone Rust service ${service_name} in ${output_dir}"