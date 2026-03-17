#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CONFIG_DIR="${LANUMINOUS_CONFIG_DIR:-$ROOT_DIR/examples/site}"
CLEAN_BUILD=false

while [[ $# -gt 0 ]]; do
  case "$1" in
    --clean)
      CLEAN_BUILD=true
      shift
      ;;
    *)
      echo "Unknown option: $1" >&2
      echo "Usage: $0 [--clean]" >&2
      exit 1
      ;;
  esac
done

echo "Building and starting Lanuminous in Docker..."
cd "$ROOT_DIR"

if [[ "$CLEAN_BUILD" == "true" ]]; then
  echo "Removing existing controller container and rebuilding without cache..."
  LANUMINOUS_CONFIG_DIR="$CONFIG_DIR" docker compose down
  LANUMINOUS_CONFIG_DIR="$CONFIG_DIR" docker compose build --no-cache
  LANUMINOUS_CONFIG_DIR="$CONFIG_DIR" docker compose up -d
else
  LANUMINOUS_CONFIG_DIR="$CONFIG_DIR" docker compose up --build -d
fi

echo "Lanuminous is starting in the background on http://127.0.0.1:9097"
echo "View logs with: docker compose logs -f controller"
