mod cmd;
mod config;
mod error;

use config::Config;
use dialoguer::Confirm;
use error::{Error, Result};
use manaba_sdk::{Client, Cookie, error::ManabaError};

#[tokio::main]
async fn main() -> Result<()> {
    cmd::cmd().await?;
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
        Err(e) => return Err(e),
    };
    Ok(config)
}

pub async fn client(config: &mut Config) -> Result<Client> {
    loop {
        let cookie = Cookie::load(&config.cookie_domain)?;

        match Client::new(&config.base_url, &cookie).await {
            Ok(client) => return Ok(client),
            Err(ManabaError::InvalidCookie) => {
                let confirmation = Confirm::new()
                    .with_prompt("Cookie is invalid. Open manaba to load new Cookie?")
                    .interact()
                    .unwrap();

                if !confirmation {
                    std::process::exit(0);
                }

                opener::open(&config.base_url)?;

                let confirmation = Confirm::new()
                    .with_prompt("Load cookie? (Yes after opening manaba)")
                    .interact()
                    .unwrap();

                if !confirmation {
                    std::process::exit(0);
                }

                continue;
            }
            Err(e) => return Err(Error::from(e)),
        }
    }
}
