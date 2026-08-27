#!/usr/bin/env bash
set -euo pipefail

if (($# != 2)); then
  echo "usage: $0 <service-directory> <output-directory>" >&2
  exit 2
fi

service_dir="${1%/}"
output_dir="${2%/}"
service_name="$(basename "${service_dir}")"

for file in Cargo.toml Makefile make.generated.mk Dockerfile \
  docker-compose.generated.yml docker-compose.dev.generated.yml \
  dependency-download-env.generated.sh dependency-download-mirrors.generated.env \
  dependency-download-mirrors.env .dockerignore .gitignore README.md; do
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
mv "${output_dir}/docker-compose.generated.yml" "${output_dir}/docker-compose.yml"

echo "Packaged standalone Rust service ${service_name} in ${output_dir}"