use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "neux")]
#[command(about = "LSFS - Semantic file search for developers", long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Index(Index),
    Search(Search),
}

#[derive(Parser)]
pub struct Index {
    #[arg(help = "Directory to index")]
    pub path: std::path::PathBuf,
    
    #[arg(short, long, default_value = "4")]
    pub batch_size: usize,
}

#[derive(Parser)]
pub struct Search {
    #[arg(help = "Search query")]
    pub query: String,
    
    #[arg(short, long, default_value_t = 10)]
    pub top_k: usize,
}