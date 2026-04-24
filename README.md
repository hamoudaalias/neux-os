# NUX OS - Technical Documentation

## About NUX OS

**Creator**: Hamouda ALIAS  
**Language**: Rust  
**License**: GPLv2 (kernel) + Apache 2.0 (userspace)  
**Version**: 1.9  
**Status**: Prototype / Documentation

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
neux search <query>     # Semantic search
```

## Tech Stack

- Rust (performance, memory safety)
- FAISS (vector search)
- sentence-transformers (embeddings)

## Roadmap

| Version | Objective |
|---------|----------|
| v0.1 | PoC CLI basic |
| v0.2 | Reranking |
| v1.0 | Release |

## Learn More

See full technical documentation in `docs/NUX.md`.

---

**Creator**: Hamouda ALIAS  
**Contact**: [to be completed]
