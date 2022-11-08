use crate::{cli::Cli, error::Error};

use clap::Parser;
use tracing::Level;

mod cli;
mod error;
mod eth;
mod ipfs;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let cli = Cli::parse();
    if cli.debug {
        tracing_subscriber::fmt().with_max_level(Level::INFO).init();
    }
    cli.run().await
}
