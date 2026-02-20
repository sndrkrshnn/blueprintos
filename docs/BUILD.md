# Building MuninOS (distro core)

## Host requirements (Debian/Ubuntu)
```bash
sudo apt update
sudo apt install -y build-essential git bc bison flex libssl-dev libelf-dev \
  debootstrap squashfs-tools xorriso grub-pc-bin grub-efi-amd64-bin mtools \
  rsync cpio dosfstools qemu-system-x86
```

## Build flow
```bash
make rootfs
make iso
make qemu
```

Artifacts:
- `build/live/vmlinuz`
- `build/live/initrd.img`
- `build/live/filesystem.squashfs`
- `build/muninos-dev.iso`

## Optional custom kernel
```bash
make kernel
make rootfs
make iso
```
If `build/kernel/bzImage` exists, ISO build prefers it as kernel payload.
