mod cmd;
mod config;
mod error;

use clap::{Parser, Subcommand};
use config::Config;
use error::{Error, Result};
use manaba_sdk::{Client, Cookie};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
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

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Report { all } => {
            let mut config = config().await?;
            let client = client(&mut config).await?;
            cmd::report(&client, all).await?;
        }

        Commands::Exam { all } => {
            let mut config = config().await?;
            let client = client(&mut config).await?;
            cmd::exam(&client, all).await?;
        }

        Commands::Check { all } => {
            let mut config = config().await?;
            let client = client(&mut config).await?;

            println!("=====Report\n");
            cmd::report(&client, all).await?;

            println!("=====Exam\n");
            cmd::exam(&client, all).await?;
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

pub async fn config() -> Result<Config> {
    let config = match Config::from_file().await {
        Ok(v) => v,
        Err(Error::ConfigFileNotFound) => {
            let new_config = Config::default();
            new_config.clone().save_to_file().await?;
            new_config
        }
        Err(error) => panic!("Failed to manage config file: {error}"),
    };
    Ok(config)
}

pub async fn client(config: &mut Config) -> Result<Client> {
    let cookie = Cookie::load(&config.cookie_domain)?;
    let client = match Client::new(&config.base_url, &cookie).await {
        Ok(client) => client,
        Err(_) => {
            panic!("Cookie is invalid");
        }
    };

    Ok(client)
}
