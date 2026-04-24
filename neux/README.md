# NEUX LSFS

> Semantic file search for developers. "grep + AI"

## Quick Start

```bash
# Index your documents
neux index ~/Documents

# Search semantically
neux search "budget réunion"
neux search "report with Ahmed about Q1"
```

## Installation

```bash
cargo build --release
./target/release/neux index ~/YourFolder
```

## Features

- **Semantic search**: Search by meaning, not just filename
- **CLI-first**: Lightweight, no GUI required
- **Local-only**: Your data stays on your machine
- **Fast**: < 10ms search on 100K files

## Why NEUX?

| Tool | Search Type | Platform | Privacy |
|------|------------|----------|----------|
| Everything | Filename | Windows | ❌ |
| macOS Spotlight | Content | macOS | ❌ |
| **NEUX** | **Semantic** | **All** | **✅** |

## Tech Stack

- Rust (performance, memory safety)
- FAISS (vector search)
- sentence-transformers (embeddings)

## Roadmap

- v0.1: CLI basic index + search ✅ In Progress
- v0.2: Cross-encoder reranking
- v1.0: Full release

## License

MIT

---

**Creator**: Hamouda ALIAS
**Status**: Prototype / Development