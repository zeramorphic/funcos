#![no_std]
#![no_main]
#![feature(custom_test_frameworks, abi_x86_interrupt)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

pub mod colour;
pub mod gdt;
pub mod interrupts;
pub mod linalg;
pub mod num_traits;
pub mod print;
pub mod qemu;
pub mod screen_font;
pub mod serial;
pub mod terminal_video;
pub mod video;

use colour::Colour;
use terminal_video::TerminalVideoBuffer;

const CONFIG: bootloader_api::BootloaderConfig = {
    let mut config = bootloader_api::BootloaderConfig::new_default();
    config.kernel_stack_size = 100 * 1024; // 100 KiB
    config
};

bootloader_api::entry_point!(kmain, config = &CONFIG);

fn kmain(boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    serial_println!("\n---\nFuncOS kernel main function called.\n---");

    if let Some(framebuffer) = boot_info.framebuffer.take() {
        terminal_video::TerminalVideoBuffer::new(video::VideoBuffer::from_bootloader(framebuffer))
            .make_default();
    }

    serial_println!("Framebuffer obtained.");

    gdt::init();
    interrupts::init_idt();

    serial_println!("GDT and IDT loaded.");

    TerminalVideoBuffer::with_default(|terminal| {
        terminal.set_background(Colour::from_rgb(10, 15, 20));
        terminal.clear_screen();
    });

    // Print the system memory layout.
    for region in boot_info.memory_regions.iter() {
        println!(
            "Region: {:p} - {:p} ({:?})",
            region.start as *const (), region.end as *const (), region.kind
        );
    }

    #[cfg(test)]
    {
        test_main();
        qemu::exit_qemu(qemu::QemuExitCode::Success);
    }

    #[cfg(not(test))]
    {
        println!("Hello, world! 0.1 + 0.2 = {}", 0.1 + 0.2);
        println!("Testing enabled: {}", cfg!(test));
        panic!("Shutting down kernel.");
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    // First, print the panic info to the serial output
    // so that we can see the error even if the OS crashes.
    serial_println!("{}", info);

    unsafe {
        TerminalVideoBuffer::with_default_unchecked(|terminal| {
            use core::fmt::Write;

            terminal.set_background(Colour::BLACK);
            terminal.set_foreground(Colour::RED);

            // Ignore any errors produced here - we're too far gone to recover at this point.
            let _ = writeln!(terminal, "{info}");
        });
    }

    if cfg!(test) {
        qemu::exit_qemu(qemu::QemuExitCode::Failed);
    } else {
        loop {
            unsafe {
                core::arch::asm!("hlt");
            }
        }
    }
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests.", tests.len());
    for (i, test) in tests.iter().enumerate() {
        serial_println!("* [{}/{}]", i + 1, tests.len());
        test();
    }
    serial_println!("Tests finished!");
    qemu::exit_qemu(qemu::QemuExitCode::Success);
}
