OUTPUT_FORMAT("binary")
ENTRY(_start)

SECTIONS {
    . = 0x00100000;

    .text : {
        KEEP(*(.multiboot))
        *(.text)
    }

    .rodata : {
        *(.rodata)
    }

    .data : {
        *(.data)
    }

    .bss : {
        *(.bss)
    }
}