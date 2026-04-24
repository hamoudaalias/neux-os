#!/bin/bash
# Build script for NEUX OS

echo "=== Installing build tools ==="
sudo apt update
sudo apt install -y nasm qemu-system-x86 curl

echo "=== Installing Rust ==="
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"

echo "=== Adding x86_64-none target ==="
rustup target add x86_64-unknown-none

echo "=== Building NEUX OS ==="
cd "$(dirname "$0")"

# Build bootloader
echo "Building bootloader..."
nasm -f bin boot.asm -o boot.bin

# Build kernel  
echo "Building kernel..."
cargo build --release --target x86_64-unknown-none

echo "=== Creating disk image ==="
cat boot.bin target/x86_64-unknown-none/release/neux-os > neux.img
truncate -s 1M neux.img

echo "=== Done! Files created:"
ls -la *.bin *.img

echo ""
echo "=== To run with QEMU ==="
echo "qemu-system-x86_64 -drive format=raw,file=neux.img"