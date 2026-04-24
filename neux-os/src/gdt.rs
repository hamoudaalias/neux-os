//! GDT - Global Descriptor Table with proper selectors

pub const CODE_SEG: u16 = 0x08;
pub const DATA_SEG: u16 = 0x10;

#[repr(packed)]
pub struct Descriptor {
    limit_low: u16,
    base_low: u16,
    base_mid: u8,
    access: u8,
    flags_limit: u8,
    base_high: u8,
}

impl Descriptor {
    pub fn code() -> Self {
        Self {
            limit_low: 0xFFFF,
            base_low: 0,
            base_mid: 0,
            access: 0x9A, // Present=1, DPL=00, System=0, Type=1010(Code)
            flags_limit: 0xAF, // Flags=1011(32-bit, 4K pages), Limit=1111
            base_high: 0,
        }
    }

    pub fn data() -> Self {
        Self {
            limit_low: 0xFFFF,
            base_low: 0,
            base_mid: 0,
            access: 0x92, // Present=1, DPL=00, System=0, Type=0010(Data)
            flags_limit: 0xCF, // Flags=1100(32-bit, 4K pages), Limit=1111
            base_high: 0,
        }
    }
}

pub fn init() {
    // Load GDT
    let gdt_addr = &GDT as *const _ as u64;
    
    unsafe {
        core::arch::asm!(
            "lgdt [gdtr]",
            // Reload segment registers
            "mov ax, 0x10",  // Data segment selector
            "mov ds, ax",
            "mov es, ax",  
            "mov fs, ax",
            "mov gs, ax",
            "mov ss, ax",
            // CS is set via far jump
            "push 0x08",
            "lea rax, [rip + 1f]",
            "push rax",
            "retf",
            "1:",
            options(nostack)
        );
    }
}

static mut GDT: [Descriptor; 3] = [
    // Null descriptor (required)
    Descriptor {
        limit_low: 0,
        base_low: 0,
        base_mid: 0,
        access: 0,
        flags_limit: 0,
        base_high: 0,
    },
    // Code segment
    Descriptor::code(),
    // Data segment
    Descriptor::data(),
];

// GDTR structure
#[repr(packed)]
pub struct GDTR {
    limit: u16,
    base: u64,
}

static mut GDTR: GDTR = GDTR {
    limit: (core::mem::size_of_val(&GDT) - 1) as u16,
    base: GDT.as_ptr() as u64,
};