#!/usr/bin/env bash
set -euo pipefail

host="${BACKEND_HOST:-127.0.0.1}"
port="${BACKEND_PORT:-18000}"

if lsof -nP -iTCP:"${port}" -sTCP:LISTEN >/dev/null 2>&1; then
  printf "Port %s is already in use. Set BACKEND_PORT to a free port.\n" "${port}" >&2
  exit 1
fi

exec uv run uvicorn aether_backend.main:app --app-dir backend/src --host "${host}" --port "${port}"
