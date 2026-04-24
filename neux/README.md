# neux

LSFS - Semantic file search for developers

## Usage

```bash
# Build
cargo build --release

# Index a directory
./target/release/neux index ~/Documents

# Search
./target/release/neux search "budget meeting"
```

## Commands

- `neux index <dir>` - Index files in a directory
- `neux search <query>` - Semantic search