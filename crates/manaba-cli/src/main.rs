mod cmd;
mod config;
mod error;

use config::Config;
use error::{Error, Result};
use manaba_sdk::{Client, Cookie};

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
