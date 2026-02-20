#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

echo "[ci-smoke] build + validate + iso"
bash distro/scripts/ci-build.sh

echo "[ci-smoke] qemu smoke test"
bash distro/scripts/qemu-smoke-test.sh

echo "[ci-smoke] success"
