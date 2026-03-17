#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CONFIG_DIR="${LANUMINOUS_CONFIG_DIR:-$ROOT_DIR/examples/site}"

echo "Building and starting Lanuminous in Docker..."
cd "$ROOT_DIR"
LANUMINOUS_CONFIG_DIR="$CONFIG_DIR" docker compose up --build -d

echo "Lanuminous is starting in the background on http://127.0.0.1:9097"
echo "View logs with: docker compose logs -f controller"
