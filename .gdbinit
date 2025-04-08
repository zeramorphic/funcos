file target/x86_64-funcos/debug/kernel.elf
target remote localhost:1234
break kernel::interrupts::double_fault_handler
