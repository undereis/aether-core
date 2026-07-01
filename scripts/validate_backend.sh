#!/usr/bin/env bash
set -euo pipefail

host="${BACKEND_HOST:-127.0.0.1}"
port="${BACKEND_PORT:-18000}"

curl -fsS "http://${host}:${port}/health"
curl -fsS "http://${host}:${port}/version"
