//! GDT - Global Descriptor Table

pub struct Descriptor {
    limit: u16,
    base: u32,
    access: u8,
    flags: u8,
}

impl Descriptor {
    pub fn code_segment() -> Self {
        Self {
            limit: 0xFFFF,
            base: 0,
            access: 0x9A, // Present, Ring 0, Executable, Readable
            flags: 0x0C, // Granularity, 32-bit
        }
    }

    pub fn data_segment() -> Self {
        Self {
            limit: 0xFFFF,
            base: 0,
            access: 0x92, // Present, Ring 0, Writable
            flags: 0x0C,
        }
    }
}

pub fn init() {
    // GDT is set by bootloader in real OS
    // This is where we'd load our descriptors
}

pub fn load() {
    unsafe {
        core::arch::asm!(
            "lgdt [gdtr]",
            options(nostack)
        );
    }
}

static gdtr: [u8; 16] = [0; 16];