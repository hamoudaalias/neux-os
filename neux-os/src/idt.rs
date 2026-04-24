//! IDT - Interrupt Descriptor Table

pub struct InterruptDescriptor {
    offset_low: u16,
    selector: u16,
    ist: u8,
    type_attr: u8,
    offset_mid: u16,
    offset_high: u32,
    reserved: u32,
}

pub fn init() {
    // Initialize IDT with basic exceptions
}

pub fn load() {
    unsafe {
        core::arch::asm!(
            "lidt [idtr]",
            options(nostack)
        );
    }
}

static idtr: [u8; 10] = [0; 10];