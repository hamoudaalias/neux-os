//! VGA Text Mode Driver with color support

const VIDEO_MEMORY: *mut u8 = 0xB8000 as *mut u8;
const WIDTH: usize = 80;
const HEIGHT: usize = 25;

pub fn init() {
    // Clear screen
    for i in 0..(WIDTH * HEIGHT * 2) {
        unsafe { VIDEO_MEMORY.offset(i as isize).write(0) };
    }
}

pub fn write(text: &[u8]) {
    for &byte in text {
        write_byte(byte);
    }
}

pub fn writeln(text: &[u8]) {
    write(text);
    newline();
}

pub fn set_color(fg: u8, bg: u8) {
    COLOR = (bg << 4) | (fg & 0x0F);
}

static mut COLOR: u8 = 0x07; // White on black

fn write_byte(byte: u8) {
    match byte {
        b'\n' => newline(),
        b'\0' => return,
        _ => {
            let pos = (ROW * WIDTH + COL) * 2;
            unsafe {
                VIDEO_MEMORY.offset(pos as isize).write(byte);
                VIDEO_MEMORY.offset(pos as isize + 1).write(COLOR);
            }
            COL += 1;
            if COL >= WIDTH {
                newline();
            }
        }
    }
}

fn newline() {
    COL = 0;
    if ROW < HEIGHT - 1 {
        ROW += 1;
    } else {
        scroll_up();
    }
}

fn scroll_up() {
    // Scroll screen up one line
    for row in 0..(HEIGHT - 1) {
        for col in 0..WIDTH {
            let src = ((row + 1) * WIDTH + col) * 2;
            let dst = (row * WIDTH + col) * 2;
            unsafe {
                let b = VIDEO_MEMORY.offset(src as isize).read();
                VIDEO_MEMORY.offset(dst as isize).write(b);
            }
        }
    }
    // Clear last line
    for col in 0..WIDTH {
        let pos = ((HEIGHT - 1) * WIDTH + col) * 2;
        unsafe {
            VIDEO_MEMORY.offset(pos as isize).write(b' ');
        }
    }
}

static mut ROW: usize = 0;
static mut COL: usize = 0;