//! NEUX OS - Main Kernel Entry Point
//!
//! Build with: cargo build --target x86_64-unknown-neux
//! Run with: cargo run

#![no_std]
#![feature(panic_info_message)]

mod vga;
mod gdt;
mod idt;
mod memory;

use core::panic::PanicInfo;

static HELLO: &[u8] = b"NEUX OS v0.1 - AI-Augmented Operating System\0";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Initialize VGA
    vga::init();
    vga::write(HELLO);
    
    // Initialize GDT
    gdt::init();
    
    // Initialize IDT
    idt::init();
    
    // Enable interrupts
    unsafe { core::arch::asm!("sti") };
    
    loop {
        unsafe { core::arch::asm!("hlt") };
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let msg = b"PANIC: ";
    vga::write(msg);
    if let Some(s) = info.message() {
        vga::write(s.as_bytes());
    }
    loop {}
}