#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

echo "[ci-build] step 1/4: build binaries"
bash distro/scripts/build-munin-binaries.sh

echo "[ci-build] step 2/4: build rootfs"
bash distro/scripts/build-rootfs.sh

echo "[ci-build] step 3/4: validate image contents"
bash distro/scripts/validate-image.sh

echo "[ci-build] step 4/4: build iso"
bash distro/scripts/build-iso.sh

if [[ -f "$ROOT/build/muninos-dev.iso" ]]; then
  echo "[ci-build] success: $ROOT/build/muninos-dev.iso"
else
  echo "[ci-build] ERROR: ISO missing after build" >&2
  exit 1
fi
