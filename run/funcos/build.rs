use std::path::PathBuf;

use bootloader::BootConfig;

fn main() {
    let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    let kernel = PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap())
        .join("..")
        .join("..")
        .join("os")
        .join("target")
        .join("x86_64-funcos")
        .join("debug")
        .join("kernel.elf");
    println!("cargo::rerun-if-changed={}", kernel.display());

    let config = BootConfig::default();

    let uefi_path = out_dir.join("uefi.img");
    bootloader::UefiBoot::new(&kernel)
        .set_boot_config(&config)
        .create_disk_image(&uefi_path)
        .unwrap();

    let bios_path = out_dir.join("bios.img");
    bootloader::BiosBoot::new(&kernel)
        .set_boot_config(&config)
        .create_disk_image(&bios_path)
        .unwrap();

    println!("cargo:rustc-env=UEFI_PATH={}", uefi_path.display());
    println!("cargo:rustc-env=BIOS_PATH={}", bios_path.display());
}
