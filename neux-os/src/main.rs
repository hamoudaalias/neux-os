//! NEUX OS v0.2 - Kernel with exception handlers
//!
//! Build: cargo build
//! Run: qemu-system-x86_64 -drive format=raw,file=kernel.bin

#![no_std]
#![feature(panic_info_message)]

mod vga;
mod gdt;
mod idt;
mod memory;

use core::panic::PanicInfo;

const VERSION: &[u8] = b"NEUX OS v0.2 - AI-Augmented OS\0";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // 1. Init VGA
    vga::init();
    vga::writeln(VERSION);
    vga::writeln(b"Initializing...");
    
    // 2. Init GDT (memory segments)
    vga::writeln(b"Loading GDT...");
    gdt::init();
    vga::writeln(b"OK");
    
    // 3. Init IDT (interrupt handlers)
    vga::writeln(b"Loading IDT...");
    idt::init();
    vga::writeln(b"OK");
    
    // 4. Init Memory
    vga::writeln(b"Initializing memory...");
    memory::init();
    vga::writeln(b"OK");
    
    vga::writeln(b"");
    vga::writeln(b"NEUX OS Ready!");
    vga::writeln(b"Type 'help' for commands");
    
    // Enable interrupts
    unsafe { core::arch::asm!("sti") };
    
    // Halt with interrupts enabled
    loop {
        unsafe { core::arch::asm!("hlt") };
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let panic_msg = b"PANIC: \0";
    vga::set_color(0x0C, 0x00); // Red on black
    vga::writeln(panic_msg);
    
    if let Some(s) = info.message() {
        vga::write(s.as_bytes());
    }
    loop {}
}