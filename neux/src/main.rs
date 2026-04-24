mod cli;
mod indexer;
mod searcher;

fn main() -> anyhow::Result<()> {
    cli::run()
}