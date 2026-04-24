//! IDT - Interrupt Descriptor Table with handlers

pub const DIVIDE_ERROR: u8 = 0;
pub const DEBUG: u8 = 1;
pub const NMI: u8 = 2;
pub const BREAKPOINT: u8 = 3;
pub const OVERFLOW: u8 = 4;
pub const BOUNDS: u8 = 5;
pub const INVALID_OP: u8 = 6;
pub const NO_COPROC: u8 = 7;
pub const DOUBLE_FAULT: u8 = 8;
pub const COPROC_SEG: u8 = 9;
pub const INVALID_TSS: u8 = 10;
pub const SEG_NOT_PRES: u8 = 11;
pub const STACK_FAULT: u8 = 12;
pub const PROTECTION: u8 = 13;
pub const PAGE_FAULT: u8 = 14;
pub const RESERVED: u8 = 15;

#[repr(packed)]
pub struct IdtEntry {
    offset_low: u16,
    selector: u16,
    ist: u3,
    _type_attr: u8,
    offset_mid: u16,
    offset_high: u32,
    reserved: u32,
}

impl IdtEntry {
    pub fn new(handler: u64, selector: u16) -> Self {
        Self {
            offset_low: handler as u16,
            selector,
            ist: 0,
            _type_attr: 0x8E, // Present=1, DPL=00, Type=1110(32-bit Interrupt Gate)
            offset_mid: (handler >> 16) as u16,
            offset_high: (handler >> 32) as u32,
            reserved: 0,
        }
    }
}

pub fn init() {
    // Set up IDT
    // In full OS, would set up interrupt handlers
    
    unsafe {
        core::arch::asm!("lidt [idtr]", options(nostack));
    }
}

pub fn handle_divide_error() {
    let msg = b"Divide Error!\0";
    crate::vga::write(msg);
}

pub fn handle_page_fault() {
    let msg = b"Page Fault!\0";
    crate::vga::write(msg);
    loop {}
}

pub fn handle_general_protection() {
    let msg = b"General Protection Fault!\0";
    crate::vga::write(msg);
    loop {}
}

// IDTR
static mut IDTR: [u8; 6] = [0; 6];