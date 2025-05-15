mod exam;
mod report;
mod timetable;

use crate::{APP_CONFIG, APP_CONFIG_PATH, client, color::AppColorize as _, error::Result};
use clap::{Parser, Subcommand};
use exam::exam;
use manaba_sdk::assignment::AssignmentDate;
use manaba_sdk::assignment::{AssignmentImportanceLevel, AssignmentReceptibleState};
use report::report;
use timetable::timetable;

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
    /// Show manaba-cli config path
    ConfigPath,
    /// Show timetable
    Timetable,
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
            let app_config = APP_CONFIG.get().unwrap();
            let client = client(app_config).await?;

            report::report(&client, all, warn).await?;
        }

        Commands::Exam { all, warn } => {
            let app_config = APP_CONFIG.get().unwrap();
            let client = client(app_config).await?;

            exam(&client, all, warn).await?;
        }

        Commands::Check { all, warn } => {
            let app_config = APP_CONFIG.get().unwrap();
            let client = client(app_config).await?;

            println!(
                "============ {} ============\n",
                " Report ".black().on_white()
            );
            report(&client, all, warn).await?;

            println!(
                "============ {} ============\n",
                " Exam ".black().on_white()
            );
            exam(&client, all, warn).await?;
        }

        Commands::Browse => {
            let app_config = APP_CONFIG.get().unwrap();
            opener::open(&app_config.base_url)?;
        }

        Commands::Timetable => {
            let app_config = APP_CONFIG.get().unwrap();
            timetable(&app_config.timetable);
        }

        Commands::ConfigPath => {
            println!("{:?}", APP_CONFIG_PATH.get().unwrap());
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
        return text.gray();
    }

    match importance_level {
        AssignmentImportanceLevel::High => text.red(),
        AssignmentImportanceLevel::Medium => text.yellow(),
        AssignmentImportanceLevel::Low => text.aqua(),
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
        return text.gray();
    }

    match importance_level {
        AssignmentImportanceLevel::High => text.on_red(),
        AssignmentImportanceLevel::Medium => text.on_yellow(),
        AssignmentImportanceLevel::Low => text.on_aqua(),
        AssignmentImportanceLevel::None => text.to_string(),
    }
}

fn date_as_str(report_date: &AssignmentDate) -> String {
    report_date.date.format("%Y-%m-%d %H:%M").to_string()
}
