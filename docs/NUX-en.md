# NUX OS - Technical Documentation

## About NUX OS

**Creator**: Hamouda ALIAS  
**Language**: Rust  
**License**: GPLv2 (kernel) + Apache 2.0 (userspace)  
**Version**: 2.0  
**Status**: Public on GitHub (June 2026)

## Overview

NUX OS is an AI-augmented operating system project, focused primarily on LSFS (Latent Space File System) - a semantic file search for developers.

## Quick Start

```bash
cd neux
cargo build --release
./target/release/neux index ~/Documents
./target/release/neux search "budget meeting"
```

## Commands

```bash
neux index <directory>  # Index a folder
neux search <query>       # Semantic search
```

## Tech Stack

- Rust (performance, memory safety)
- FAISS (vector search)
- sentence-transformers (embeddings)

## Architecture

The project follows a hybrid architecture:

1. **GPU-First**: CUDA/ROCm for embeddings
2. **NPU** (optional): Bonus for faster inference
3. **CPU Fallback**: Classic mode for older hardware

## LSFS (Latent Space File System)

LSFS is the core feature - semantic file search:

- **Layer 1**: Semantic embeddings (shadow metadata)
- **Layer 2**: Binary storage (files remain intact)
- **Search**: Semantic queries, not just filenames

## Security

- **Encrypted-at-Rest**: AES-256-GCM for embeddings
- **Session-based**: Decryption only in VRAM during active session
- **Intent Firewall**: Permission Granular controls
- **No Swap**: Decrypted embeddings never swapped to disk

## Roadmap

| Version | Objective |
|---------|----------|
| v0.1 | PoC CLI basic |
| v0.2 | Reranking |
| v1.0 | Full Release |

## Links

- **GitHub**: https://github.com/hamoudaalias/neux-os
- **Technical Docs**: See `docs/NUX.md`

---

**Creator**: Hamouda ALIAS