#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
BUILD="$ROOT/build"
ISO_WORK="$ROOT/workdir/iso"
GRUB_CFG="$ROOT/distro/iso/grub/grub.cfg"

mkdir -p "$ISO_WORK/boot/grub" "$ISO_WORK/live" "$BUILD"
cp "$GRUB_CFG" "$ISO_WORK/boot/grub/grub.cfg"

for f in initrd.img filesystem.squashfs; do
  if [[ ! -f "$BUILD/live/$f" ]]; then
    echo "[iso] Missing artifact: $BUILD/live/$f"
    exit 1
  fi
  cp "$BUILD/live/$f" "$ISO_WORK/live/$f"
done

# Prefer custom kernel when available
if [[ -f "$BUILD/live/vmlinuz-custom" ]]; then
  cp "$BUILD/live/vmlinuz-custom" "$ISO_WORK/live/vmlinuz"
  echo "[iso] Using custom kernel: $BUILD/live/vmlinuz-custom"
elif [[ -f "$BUILD/live/vmlinuz" ]]; then
  cp "$BUILD/live/vmlinuz" "$ISO_WORK/live/vmlinuz"
  echo "[iso] Using distro kernel: $BUILD/live/vmlinuz"
else
  echo "[iso] Missing kernel artifact (vmlinuz or vmlinuz-custom)"
  exit 1
fi

if command -v grub-mkrescue >/dev/null 2>&1; then
  grub-mkrescue -o "$BUILD/muninos-dev.iso" "$ISO_WORK"
else
  echo "[iso] grub-mkrescue not found. Install grub-pc-bin + grub-efi-amd64-bin + xorriso"
  exit 1
fi

echo "[iso] Done -> $BUILD/muninos-dev.iso"
