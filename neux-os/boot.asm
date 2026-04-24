; NEUX OS - Bootstrap (16-bit Real Mode)
; Assemble with: nasm -f bin boot.asm -o boot.bin
; Run with: qemu-system-x86_64 -drive format=raw,file=boot.bin

ORG 0x7C00

start:
    mov ax, 0x07E0    ; Setup stack
    mov ss, ax
    mov sp, 0x1000
    
    mov ax, 0xB800     ; VGA segment
    mov ds, ax
    
    mov si, msg
    call print
    
.loop:
    hlt
    jmp .loop

print:
    lodsb
    or al, al
    jz .done
    mov ah, 0x0E
    int 0x10
    jmp print
.done:
    ret

msg db "NEUX OS v0.1 - Bootloader OK", 0x0D, 0x0A, 0

; Boot sector signature
times 510-($-$$) db 0
dw 0xAA55