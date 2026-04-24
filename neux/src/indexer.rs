use anyhow::Result;
use walkdir::WalkDir;
use std::collections::HashMap;

pub struct Indexer {
    files: Vec<FileEntry>,
    pub index_path: String,
}

pub struct FileEntry {
    pub path: String,
    pub hash: String,
    pub size: u64,
    pub modified: u64,
}

impl Indexer {
    pub fn new() -> Self {
        Self {
            files: Vec::new(),
            index_path: ".neux/index".to_string(),
        }
    }

    pub fn index_directory(&mut self, path: &std::path::Path) -> Result<usize> {
        let mut count = 0;
        
        for entry in WalkDir::new(path)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    let ext = ext.to_string_lossy().to_lowercase();
                    if self.is_supported(&ext) {
                        let metadata = std::fs::metadata(path)?;
                        self.files.push(FileEntry {
                            path: path.to_string_lossy().to_string(),
                            hash: String::new(),
                            size: metadata.len(),
                            modified: metadata.modified()?.timestamp() as u64,
                        });
                        count += 1;
                    }
                }
            }
        }
        
        Ok(count)
    }

    fn is_supported(&self, ext: &str) -> bool {
        matches!(ext.as_str(), 
            "txt" | "md" | "rs" | "py" | "js" | "ts" 
            | "json" | "yaml" | "yml" | "toml" | "html" | "css"
        )
    }

    pub fn save(&self) -> Result<()> {
        std::fs::create_dir_all(&self.index_path)?;
        let index_file = format!("{}/files.json", self.index_path);
        let json = serde_json::to_string_pretty(&self.files)?;
        std::fs::write(index_file, json)?;
        Ok(())
    }
}

pub fn run_index(cmd: crate::cli::IndexCmd) -> Result<()> {
    println!("📁 Indexing: {}", cmd.path.display());
    
    let mut indexer = Indexer::new();
    let count = indexer.index_directory(&cmd.path)?;
    
    println!("✅ Found {} files", count);
    indexer.save()?;
    println!("💾 Index saved to: {}", indexer.index_path);
    
    Ok(())
}