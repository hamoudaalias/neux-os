# NEUX OS

> AI-Augmented Operating System built from scratch

## Build from Scratch

This is a true **from-scratch operating system** written in Rust.

### Architecture

```
┌──────────────────────────┐
│      User Space         │
├──────────────────────────┤
│   System Calls        │
├──────────────────────────┤
│    File System      │
├──────────────────────────┤
│  Memory Manager    │
├──────────────────────────┤
│     Kernel        │
├──────────────────────────┤
│    Bootloader     │
└──────────────────────────┘
```

### Components

| Component | Status | Description |
|-----------|--------|------------|
| Bootloader | ✅ | 16-bit real mode |
| Kernel | ✅ | Basic entry point |
| VGA | ✅ | Text mode output |
| GDT | 🔄 | Segments |
| IDT | 🔄 | Interrupts |
| Memory | 🔄 | Paging |
| FileSystem | ⏳ | Simple FS |

### Build

```bash
# Build kernel (requires x86_64 target)
cargo build --release

# Build bootloader
nasm -f bin boot.asm -o boot.bin

# Create disk image
cat boot.bin kernel.bin > os.bin
```

### Run

```bash
qemu-system-x86_64 -drive format=raw,file=os.bin
```

## Roadmap

| Version | Feature |
|---------|----------|
| v0.1 | Boot + VGA |
| v0.2 | GDT + IDT |
| v0.3 | Paging |
| v0.4 | Simple FS |
| v1.0 | Full OS |

---

**Creator**: Hamouda ALIAS  
**Language**: Rust + Assembly