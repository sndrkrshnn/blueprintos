# MuninOS

MuninOS is a **standalone Linux distribution** project (not an app running inside another OS).

## Core direction
- Own distro structure (`distro/`)
- Custom kernel config (`distro/kernel/configs/munin_defconfig`)
- Rootfs build pipeline (Debian base via `debootstrap`)
- Bootable ISO pipeline (GRUB + `grub-mkrescue`)
- First-party OS services (`munin-core`, `munin-sts`, `munin-ui`)

## Build quickstart (Debian/Ubuntu host)
```bash
sudo apt update
sudo apt install -y build-essential git bc bison flex libssl-dev libelf-dev \
  debootstrap squashfs-tools xorriso grub-pc-bin grub-efi-amd64-bin mtools \
  rsync cpio dosfstools qemu-system-x86

make rootfs
make iso
make qemu
```

Artifacts:
- `build/live/vmlinuz`
- `build/live/initrd.img`
- `build/live/filesystem.squashfs`
- `build/muninos-dev.iso`

### Custom kernel integration
If `build/kernel/bzImage` exists (from `make kernel`), ISO build will prefer it automatically as `/live/vmlinuz`.

## Repo structure
```text
.
├── distro/
│   ├── kernel/configs/munin_defconfig
│   ├── rootfs/
│   │   ├── overlay/
│   │   └── packages/base.txt
│   ├── iso/grub/grub.cfg
│   └── scripts/
│       ├── build-kernel.sh
│       ├── build-rootfs.sh
│       ├── build-iso.sh
│       └── run-qemu.sh
├── blueprint-core/
├── blueprint-sts/
├── blueprint-ui/
└── Makefile
```

## Status
This push focuses on **distro core bring-up**: boot artifacts, ISO generation, and VM boot testing path.
