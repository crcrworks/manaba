mod app_config;
mod cmd;
mod color;
mod error;

use app_config::AppConfig;
use color::{APP_COLOR, AppColor};
use config::Config;
use dialoguer::Confirm;
use error::{Error, Result, print_err};
use manaba_sdk::{Client, Cookie, error::ManabaError};
use std::{io::Write as _, path::PathBuf, sync::OnceLock};

static APP_CONFIG: OnceLock<AppConfig> = OnceLock::new();
static APP_CONFIG_PATH: OnceLock<PathBuf> = OnceLock::new();

#[tokio::main]
async fn main() -> Result<()> {
    APP_CONFIG_PATH.get_or_init(app_config_path);
    APP_CONFIG.get_or_init(|| match app_config() {
        Ok(app_config) => app_config,
        Err(e) => {
            print_err(e.to_string());
            match e {
                Error::ConfigFileNotFound { .. } => {
                    create_config_file().unwrap_or_else(|_| AppConfig::default())
                }
                Error::ConfigFileDeserialize { .. } => AppConfig::default(),
                _ => AppConfig::default(),
            }
        }
    });

    APP_COLOR.get_or_init(|| {
        let app_conig = APP_CONFIG.get().unwrap();
        let color = &app_conig.color;

        let mut app_color = AppColor::default();

        app_color.white = color.get("white").cloned().unwrap_or(app_color.white);
        app_color.black = color.get("black").cloned().unwrap_or(app_color.black);
        app_color.red = color.get("red").cloned().unwrap_or(app_color.red);
        app_color.blue = color.get("blue").cloned().unwrap_or(app_color.blue);
        app_color.aqua = color.get("aqua").cloned().unwrap_or(app_color.aqua);
        app_color.yellow = color.get("yellow").cloned().unwrap_or(app_color.yellow);
        app_color.green = color.get("green").cloned().unwrap_or(app_color.green);
        app_color.gray = color.get("gray").cloned().unwrap_or(app_color.gray);

        app_color
    });

    cmd::cmd().await?;
    Ok(())
}

fn app_config_path() -> PathBuf {
    let config_dir_path = dirs::config_dir().unwrap();
    config_dir_path.join("manaba").join("settings.toml")
}

fn app_config() -> Result<AppConfig> {
    let config = Config::builder()
        .add_source(config::File::from(
            APP_CONFIG_PATH.get().unwrap().to_owned(),
        ))
        .add_source(config::Environment::with_prefix("APP"))
        .build()
        .map_err(|e| Error::ConfigFileNotFound {
            source: e,
            config_path: APP_CONFIG_PATH.get().unwrap().to_owned(),
        })?;

    config
        .try_deserialize()
        .map_err(|e| Error::ConfigFileDeserialize {
            source: e,
            config_path: APP_CONFIG_PATH.get().unwrap().to_owned(),
        })
}

async fn client(app_config: &AppConfig) -> Result<Client> {
    loop {
        let cookie = Cookie::load(&app_config.cookie_domain)?;

        match Client::new(&app_config.base_url, &cookie).await {
            Ok(client) => return Ok(client),
            Err(ManabaError::InvalidCookie) => {
                let confirmation = Confirm::new()
                    .with_prompt("Cookie is invalid. Open manaba to load new Cookie?")
                    .interact()
                    .unwrap();

                if !confirmation {
                    std::process::exit(0);
                }

                opener::open(&app_config.base_url)?;

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

fn create_config_file() -> Result<AppConfig, ()> {
    let confirmation = Confirm::new()
        .with_prompt("Config file not found. Do you want to create a new one?")
        .interact()
        .unwrap();

    if confirmation {
        let path = APP_CONFIG_PATH.get().unwrap();

        if let Err(e) = std::fs::File::create(path).and_then(|mut file| {
            let app_config = AppConfig::default();
            let toml = toml::to_string(&app_config).unwrap();

            file.write_all(toml.as_bytes())
        }) {
            print_err(e.to_string());
        } else if let Some(path_str) = path.to_str() {
            println!("Config file created at {path_str}");
        }

        Ok(app_config().unwrap())
    } else {
        Err(())
    }
}
