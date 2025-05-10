mod exam;
mod report;

use crate::{client, config, config::Config, error::Result};
use clap::{Parser, Subcommand};
use colored_text::Colorize;
use exam::exam;
use manaba_sdk::assignment::AssignmentDate;
use manaba_sdk::assignment::{AssignmentImportanceLevel, AssignmentReceptibleState};
use report::report;

const INDENT: &str = "   ";

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
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

            println!("============ {} ============\n", " Report ".on_white());
            report(&client, all, warn).await?;

            println!("============ {} ============\n", " Exam ".on_white());
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

fn colorize<T: std::fmt::Display>(
    text: T,
    receptiable_state: &AssignmentReceptibleState,
    importance_level: &AssignmentImportanceLevel,
) -> String {
    if matches!(
        *receptiable_state,
        AssignmentReceptibleState::NotStarted | AssignmentReceptibleState::Closed
    ) {
        return text.white();
    }

    match importance_level {
        AssignmentImportanceLevel::High => text.red(),
        AssignmentImportanceLevel::Medium => text.yellow(),
        AssignmentImportanceLevel::Low => text.cyan(),
        AssignmentImportanceLevel::None => text.to_string(),
    }
}

fn colorize_bg<T: std::fmt::Display>(
    text: T,
    receptiable_state: &AssignmentReceptibleState,
    importance_level: &AssignmentImportanceLevel,
) -> String {
    if matches!(
        *receptiable_state,
        AssignmentReceptibleState::NotStarted | AssignmentReceptibleState::Closed
    ) {
        return text.white();
    }

    match importance_level {
        AssignmentImportanceLevel::High => text.on_red(),
        AssignmentImportanceLevel::Medium => text.on_yellow(),
        AssignmentImportanceLevel::Low => text.on_cyan(),
        AssignmentImportanceLevel::None => text.to_string(),
    }
}

fn date_as_str(report_date: &AssignmentDate) -> String {
    report_date.date.format("%Y-%m-%d %H:%M").to_string()
}
