# Adapted from <https://github.com/jasondyoungberg/limine-rust-template>.

# Nuke built-in rules and variables.
MAKEFLAGS += -rR
.SUFFIXES:

# Convenience macro to reliably declare user overridable variables.
override USER_VARIABLE = $(if $(filter $(origin $(1)),default undefined),$(eval override $(1) := $(2)))

# Default user QEMU flags. These are appended to the QEMU command calls.
$(call USER_VARIABLE,QEMUFLAGS,-m 2G)

override IMAGE_NAME := funcos-x86_64

# Debug or release mode.
$(call USER_VARIABLE,MODE,debug)

.PHONY: all
all: build/$(IMAGE_NAME).iso

.PHONY: run
run: build/$(IMAGE_NAME).iso
	qemu-system-x86_64 \
		-M q35 \
		-cdrom build/$(IMAGE_NAME).iso \
		-boot d \
		$(QEMUFLAGS)

build/ovmf/ovmf-code-x86_64.fd:
	mkdir -p build/ovmf
	curl -Lo $@ https://github.com/osdev0/edk2-ovmf-nightly/releases/latest/download/ovmf-code-x86_64.fd

build/ovmf/ovmf-vars-x86_64.fd:
	mkdir -p build/ovmf
	curl -Lo $@ https://github.com/osdev0/edk2-ovmf-nightly/releases/latest/download/ovmf-vars-x86_64.fd

build/limine/limine:
	rm -rf build/limine
	git clone https://github.com/limine-bootloader/limine.git --branch=v9.x-binary --depth=1 build/limine
	$(MAKE) -C build/limine

.PHONY: kernel
kernel:
	cargo build -p kernel

build/$(IMAGE_NAME).iso: build/limine/limine kernel
	rm -rf build/iso_root
	mkdir -p build/iso_root/boot
	cp -v target/x86_64-funcos/$(MODE)/kernel.elf build/iso_root/boot/
	mkdir -p build/iso_root/boot/limine
	cp -v limine.conf build/iso_root/boot/limine/
	mkdir -p build/iso_root/EFI/BOOT
	cp -v build/limine/limine-bios.sys build/limine/limine-bios-cd.bin build/limine/limine-uefi-cd.bin build/iso_root/boot/limine/
	cp -v build/limine/BOOTX64.EFI build/iso_root/EFI/BOOT/
	cp -v build/limine/BOOTIA32.EFI build/iso_root/EFI/BOOT/
	xorriso -as mkisofs -b boot/limine/limine-bios-cd.bin \
		-no-emul-boot -boot-load-size 4 -boot-info-table \
		--efi-boot boot/limine/limine-uefi-cd.bin \
		-efi-boot-part --efi-boot-image --protective-msdos-label \
		build/iso_root -o build/$(IMAGE_NAME).iso
	./build/limine/limine bios-install build/$(IMAGE_NAME).iso
	rm -rf build/iso_root

$(IMAGE_NAME).hdd: build/limine/limine kernel
	rm -f $(IMAGE_NAME).hdd
	dd if=/dev/zero bs=1M count=0 seek=64 of=$(IMAGE_NAME).hdd
	sgdisk $(IMAGE_NAME).hdd -n 1:2048 -t 1:ef00
	./build/limine/limine bios-install $(IMAGE_NAME).hdd
	mformat -i $(IMAGE_NAME).hdd@@1M
	mmd -i $(IMAGE_NAME).hdd@@1M ::/EFI ::/EFI/BOOT ::/boot ::/boot/limine
	mcopy -i $(IMAGE_NAME).hdd@@1M kernel/bin-x86_64/kernel ::/boot
	mcopy -i $(IMAGE_NAME).hdd@@1M limine.conf ::/boot/limine
	mcopy -i $(IMAGE_NAME).hdd@@1M build/limine/limine-bios.sys ::/boot/limine
	mcopy -i $(IMAGE_NAME).hdd@@1M build/limine/BOOTX64.EFI ::/EFI/BOOT
	mcopy -i $(IMAGE_NAME).hdd@@1M build/limine/BOOTIA32.EFI ::/EFI/BOOT

.PHONY: clean
clean:
	cargo clean
	rm -rf build/iso_root build/$(IMAGE_NAME).iso

.PHONY: distclean
distclean: clean
	cargo clean
	rm -rf build
