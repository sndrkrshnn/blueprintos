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
make bins
make rootfs
make validate
make iso
make qemu
```

Or run end-to-end in one shot:
```bash
make ci
```

`make bins` compiles:
- `munin-core`
- `munin-sts`
- `munin-ui` (from `munin-ui-service`)

and stages them into `build/munin-bin/` for rootfs embedding.

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
If `build/kernel/bzImage` exists, ISO build prefers it.

## Validation checks
`make validate` ensures rootfs contains:
- `/opt/muninos/bin/munin-core`
- `/opt/muninos/bin/munin-sts`
- `/opt/muninos/bin/munin-ui`
- systemd units: `munin-core/sts/ui/firstboot.service`
- UI assets at `/opt/muninos/ui/index.html`
- `/etc/default/munin-sts`

## First boot behavior
- `munin-firstboot.service` runs `munin-firstboot-wizard` once
- captures hostname/timezone and writes `/etc/muninos/setup.env`
- enables `munin-core`, `munin-sts`, `munin-ui`

## STS key at runtime
Set in image/host:
- `/etc/default/munin-sts`
- `QWEN_API_KEY=...`
