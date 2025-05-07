use crate::{client, config, config::Config, error::Result};

mod exam;
use exam::exam;

mod report;
use report::report;

use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Browse,
    ConfigPath,
    Report {
        #[arg(long)]
        all: bool,
    },
    Exam {
        #[arg(long)]
        all: bool,
    },
    Check {
        #[arg(long)]
        all: bool,
    },
}

pub async fn cmd() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Report { all } => {
            let mut config = config().await?;
            let client = client(&mut config).await?;
            report::report(&client, all).await?;
        }

        Commands::Exam { all } => {
            let mut config = config().await?;
            let client = client(&mut config).await?;
            exam(&client, all).await?;
        }

        Commands::Check { all } => {
            let mut config = config().await?;
            let client = client(&mut config).await?;

            println!("=====Report\n");
            report(&client, all).await?;

            println!("=====Exam\n");
            exam(&client, all).await?;
        }

        Commands::Browse => {
            let config = config().await?;
            opener::open(config.base_url)?;
        }

        Commands::ConfigPath => {
            println!("{:?}", Config::file_path()?);
        }
    }

    Ok(())
}
