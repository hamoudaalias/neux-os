use anyhow::Result;
use clap::Parser;

mod cli;
mod indexer;
mod searcher;
mod config;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    
    let opts = cli::Opts::parse();
    
    match opts.command {
        cli::Command::Index(cmd) => {
            indexer::run_index(cmd).await?;
        }
        cli::Command::Search(cmd) => {
            searcher::run_search(cmd).await?;
        }
    }
    
    Ok(())
}