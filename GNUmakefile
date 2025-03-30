# Adapted from <https://github.com/jasondyoungberg/limine-rust-template>.

# Nuke built-in rules and variables.
MAKEFLAGS += -rR
.SUFFIXES:

# Convenience macro to reliably declare user overridable variables.
override USER_VARIABLE = $(if $(filter $(origin $(1)),default undefined),$(eval override $(1) := $(2)))

# Default user QEMU flags. These are appended to the QEMU command calls.
$(call USER_VARIABLE,QEMUFLAGS,-m 2G)

override IMAGE_NAME := funcos-x86_64
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
		-serial stdio \
		$(QEMUFLAGS)

.PHONY: test
test: build/$(TESTS_NAME).iso
	qemu-system-x86_64 \
		-M q35 \
		-cdrom build/$(TESTS_NAME).iso \
		-boot d \
		-serial stdio \
		$(QEMUFLAGS)

build/limine/limine:
	rm -rf build/limine
	git clone https://github.com/limine-bootloader/limine.git --branch=v9.x-binary --depth=1 build/limine
	$(MAKE) -C build/limine

.PHONY: kernel
kernel:
ifeq ($(MODE),release)
	cargo build --release -p kernel
else
	cargo build -p kernel
endif

.PHONY: kernel-tests
kernel-tests:
# First print a human-readable log.
	cargo test --no-run -p kernel
# Then let's extract the executable name.
	mv $(shell cargo test --no-run -p kernel --message-format=json \
		| grep -o 'target/x86_64-funcos/debug/deps/kernel-[0-9a-f]*\.elf' \
		| head -n 1) target/x86_64-funcos/debug/kernel-tests.elf

.PHONY: asm
asm:
ifeq ($(MODE),release)
	RUSTFLAGS="--emit asm" cargo build --release -p kernel
else
	RUSTFLAGS="--emit asm" cargo build -p kernel
endif

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

build/$(TESTS_NAME).iso: build/limine/limine kernel-tests
	rm -rf build/iso_root
	mkdir -p build/iso_root/boot
	cp -v target/x86_64-funcos/debug/kernel-tests.elf build/iso_root/boot/kernel.elf
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
		build/iso_root -o build/$(TESTS_NAME).iso
	./build/limine/limine bios-install build/$(TESTS_NAME).iso
	rm -rf build/iso_root

.PHONY: clean
clean:
	cargo clean
	rm -rf build/iso_root build/$(IMAGE_NAME).iso

.PHONY: distclean
distclean: clean
	cargo clean
	rm -rf build
