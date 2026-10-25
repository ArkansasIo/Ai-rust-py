#!/usr/bin/env sh
set -eu

BASE_URL="${RUST_API_URL:-http://127.0.0.1:8080}"
JOB_NAME="${1:-health-check}"

case "$JOB_NAME" in
  health-check)
    curl --fail --silent --show-error "$BASE_URL/health" >/dev/null
    ;;
  queue-status)
    curl --fail --silent --show-error "$BASE_URL/api/v1/tasks/status" >/dev/null
    ;;
  *)
    echo "Unknown job: $JOB_NAME" >&2
    exit 2
    ;;
esac
