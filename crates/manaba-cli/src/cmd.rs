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
    /// Open manaba page in browser
    Browse,
    /// show manaba-cli config path
    ConfigPath,
    /// List reports
    Report {
        #[arg(short, long)]
        all: bool,
        /// filter by approaching deadlines
        #[arg(short, long)]
        warn: bool,
    },
    /// List exams
    Exam {
        #[arg(short, long)]
        all: bool,
        /// filter by approaching deadlines
        #[arg(short, long)]
        warn: bool,
    },
    /// List assignment include reports and exams
    Check {
        #[arg(short, long)]
        all: bool,
        /// filter by approaching deadlines
        #[arg(short, long)]
        warn: bool,
    },
}

pub async fn cmd() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Report { all, warn } => {
            let mut config = config().await?;
            let client = client(&mut config).await?;
            report::report(&client, all, warn).await?;
        }

        Commands::Exam { all, warn } => {
            let mut config = config().await?;
            let client = client(&mut config).await?;
            exam(&client, all, warn).await?;
        }

        Commands::Check { all, warn } => {
            let mut config = config().await?;
            let client = client(&mut config).await?;

            println!("===== Report\n");
            report(&client, all, warn).await?;

            println!("===== Exam\n");
            exam(&client, all, warn).await?;
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
