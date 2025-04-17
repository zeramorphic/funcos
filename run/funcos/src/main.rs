use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    debug: bool,

    #[arg(short, long)]
    test: bool,
}

// Clippy doesn't understand that we'll add new env vars in `build.rs`.
#[allow(clippy::if_same_then_else)]
fn main() {
    let args = Args::parse();

    // read env variables that were set in build script
    let uefi_path = if args.test {
        env!("UEFI_PATH_TESTS")
    } else {
        env!("UEFI_PATH")
    };
    let bios_path = if args.test {
        env!("BIOS_PATH_TESTS")
    } else {
        env!("BIOS_PATH")
    };

    // choose whether to start the UEFI or BIOS image
    let uefi = true;

    let mut cmd = std::process::Command::new("qemu-system-x86_64");
    cmd.args(["-device", "isa-debug-exit,iobase=0xf4,iosize=0x04"]);
    cmd.args(["-serial", "stdio"]);
    cmd.args(["-D", "debug.log"]);
    cmd.args(["-d", "int"]);
    cmd.args(["-m", "128M"]);
    cmd.arg("-no-reboot");
    if !args.test {
        cmd.arg("-no-shutdown");
    }
    if args.debug {
        cmd.args(["-S", "-s"]);
    }

    if uefi {
        cmd.arg("-bios").arg(ovmf_prebuilt::ovmf_pure_efi());
        cmd.arg("-drive")
            .arg(format!("format=raw,file={uefi_path}"));
    } else {
        cmd.arg("-drive")
            .arg(format!("format=raw,file={bios_path}"));
    }
    let mut child = cmd.spawn().unwrap();
    child.wait().unwrap();
}
