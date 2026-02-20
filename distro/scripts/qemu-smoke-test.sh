#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
ARCH="${ARCH:-$(cat "$ROOT/build/arch" 2>/dev/null || echo arm64)}"
ISO="$ROOT/build/muninos-${ARCH}-dev.iso"
LOG_DIR="$ROOT/build/smoke-${ARCH}"
SERIAL_LOG="$LOG_DIR/serial.log"
TIMEOUT_SECS="${TIMEOUT_SECS:-120}"
QEMU_BIN="${QEMU_BIN:-qemu-system-x86_64}"
RAM="${RAM:-2048}"
CPUS="${CPUS:-2}"

mkdir -p "$LOG_DIR"
: > "$SERIAL_LOG"

[[ -f "$ISO" ]] || { echo "[smoke] missing ISO: $ISO (run make iso)"; exit 1; }
command -v "$QEMU_BIN" >/dev/null 2>&1 || { echo "[smoke] missing $QEMU_BIN"; exit 1; }

KVM_FLAG=""
if [[ -e /dev/kvm ]]; then
  KVM_FLAG="-enable-kvm -cpu host"
fi

echo "[smoke] booting $ISO for up to ${TIMEOUT_SECS}s..."

# Run VM headless and log serial output
set +e
timeout "$TIMEOUT_SECS" "$QEMU_BIN" \
  $KVM_FLAG \
  -m "$RAM" \
  -smp "$CPUS" \
  -nographic \
  -serial file:"$SERIAL_LOG" \
  -monitor none \
  -no-reboot \
  -boot d \
  -cdrom "$ISO" \
  -net nic -net user
QEMU_RC=$?
set -e

if [[ $QEMU_RC -ne 0 && $QEMU_RC -ne 124 ]]; then
  echo "[smoke] qemu failed rc=$QEMU_RC"
  exit 1
fi

# Basic log assertions (best-effort for early boot)
if [[ ! -s "$SERIAL_LOG" ]]; then
  echo "[smoke] serial log is empty: $SERIAL_LOG"
  exit 1
fi

PASS=0
if grep -Eiq "Linux version|Booting Linux|systemd" "$SERIAL_LOG"; then
  PASS=1
fi

if [[ "$PASS" -eq 1 ]]; then
  echo "[smoke] PASS: detected Linux/systemd boot markers"
  echo "[smoke] log: $SERIAL_LOG"
  exit 0
else
  echo "[smoke] FAIL: no expected boot markers found"
  echo "[smoke] log: $SERIAL_LOG"
  tail -n 120 "$SERIAL_LOG" || true
  exit 1
fi
