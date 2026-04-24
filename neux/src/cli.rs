use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
#[command(name = "neux")]
#[command(about = "LSFS - Semantic file search for developers", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Parser)]
pub enum Command {
    #[command(name = "index")]
    Index(IndexCmd),
    #[command(name = "search")]
    Search(SearchCmd),
}

#[derive(Parser)]
pub struct IndexCmd {
    pub path: std::path::PathBuf,
    #[arg(short, long, default_value_t = 4)]
    pub batch_size: usize,
}

#[derive(Parser)]
pub struct SearchCmd {
    pub query: String,
    #[arg(short, long, default_value_t = 10)]
    pub top_k: usize,
}

pub fn run() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Index(cmd) => indexer::run_index(cmd)?,
        Command::Search(cmd) => searcher::run_search(cmd)?,
    }
    Ok(())
}