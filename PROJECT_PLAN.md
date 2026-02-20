# MuninOS — Distro-first Plan

## Direction
MuninOS is a standalone Linux distribution with its own kernel path and native runtime services.

## Completed today
- Distro scaffolding + build scripts
- Live rootfs generation with initramfs
- ISO generation via GRUB rescue flow
- QEMU boot test script
- Service scaffolding (`munin-core`, `munin-sts`, `munin-ui`)
- Naming migration from BlueprintOS -> MuninOS

## Immediate next milestones
1. Validate boot on QEMU and fix any live-boot/initramfs issues
2. Replace placeholder service binaries with real core/sts/ui launches
3. Add first-boot provisioning (users, locale, network defaults)
4. Harden kernel profile and switch to custom kernel as default path
