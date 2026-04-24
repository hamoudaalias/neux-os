//! VGA Text Mode Driver

pub struct VgaWriter {
    col: usize,
    row: usize,
}

impl VgaWriter {
    pub fn new() -> Self {
        Self { col: 0, row: 0 }
    }

    pub fn write(&mut self, text: &[u8]) {
        for &byte in text {
            self.write_byte(byte);
        }
    }

    fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => {
                self.col = 0;
                self.row = (self.row + 1) % 25;
            }
            b'\0' => return,
            _ => {
                self.write_char(byte);
                self.col += 1;
                if self.col >= 80 {
                    self.col = 0;
                    self.row = (self.row + 1) % 25;
                }
            }
        }
    }

    fn write_char(&mut self, char: u8) {
        let offset = (self.row * 80 + self.col) * 2;
        let vga = 0xB8000usize as *mut u8;
        unsafe {
            *vga.offset(offset as isize) = char;
            *vga.offset(offset as isize + 1) = 0x07; // White on black
        }
    }
}

pub fn init() {
    // Clear screen
    let vga = 0xB8000usize as *mut u8;
    for i in 0..(80 * 25 * 2) {
        unsafe { *vga.offset(i as isize) = 0 };
    }
}

pub static mut WRITER: VgaWriter = VgaWriter::new();

pub fn write(text: &[u8]) {
    unsafe { WRITER.write(text) };
}