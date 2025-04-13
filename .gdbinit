file os/target/x86_64-funcos/debug/kernel.elf
target remote localhost:1234
# break kernel::interrupts::breakpoint_handler
break kernel::interrupts::general_protection_fault_handler
break kernel::interrupts::double_fault_handler
