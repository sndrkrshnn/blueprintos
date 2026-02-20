SHELL := /bin/bash

.PHONY: all bins kernel rootfs validate iso qemu ci clean

all: bins rootfs validate iso

bins:
	bash distro/scripts/build-munin-binaries.sh

kernel:
	bash distro/scripts/build-kernel.sh

rootfs:
	bash distro/scripts/build-rootfs.sh

validate:
	bash distro/scripts/validate-image.sh

iso: validate
	bash distro/scripts/build-iso.sh

qemu:
	bash distro/scripts/run-qemu.sh

ci:
	bash distro/scripts/ci-build.sh

clean:
	rm -rf build workdir/iso
