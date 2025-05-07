use crate::error::{Error, Result};
use dialoguer::Confirm;
use manaba_sdk::Cookie;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs::{self, File};
use tokio::io::{AsyncReadExt as _, AsyncWriteExt as _};

const BASE_URL: &str = "https://ct.ritsumei.ac.jp/ct/";
const COOKIE_DOMAIN: &str = "ct.ritsumei.ac.jp";
const TIMETABLE: &str = "";

#[derive(Deserialize, Serialize, Clone)]
pub struct Config {
    pub base_url: String,
    pub cookie_domain: String,
    pub cookie: Cookie,
    pub timetable: String,
}

impl Config {
    pub async fn from_file() -> Result<Self> {
        let config_file_path = Self::file_path()?;

        let mut file = File::open(&config_file_path)
            .await
            .map_err(|_| Error::ConfigFileNotFound)?;

        let mut config_string = String::new();

        file.read_to_string(&mut config_string).await?;

        let content =
            toml::from_str(&config_string).map_err(|e| Error::TomlParse(e.to_string()))?;

        Ok(content)
    }

    pub async fn update_cookie(&mut self) -> Result<()> {
        Confirm::new()
            .with_prompt("Enter to load new cookie")
            .interact()
            .unwrap();

        let cookie = Cookie::load(&self.cookie_domain)?;

        let mut current_config = Self::from_file().await?;
        current_config.cookie = cookie.clone();
        current_config.save_to_file().await?;
        self.cookie = cookie;

        Ok(())
    }

    pub async fn save_to_file(self) -> Result<()> {
        let config_file_path = Self::file_path()?;

        if let Some(parent_dir) = config_file_path.parent() {
            fs::create_dir_all(parent_dir).await?;
        }

        let mut file = File::create(&config_file_path)
            .await
            .map_err(Error::SaveConfigFile)?;
        let toml = toml::to_string(&self).unwrap();
        file.write_all(toml.as_bytes()).await?;
        Ok(())
    }

    pub fn file_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir().ok_or(Error::ConfigFileNotFound)?;

        let app_config_dir = config_dir.join("manaba");
        let cookie_file_path = app_config_dir.join("settings.toml");

        Ok(cookie_file_path)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            base_url: BASE_URL.to_owned(),
            cookie_domain: COOKIE_DOMAIN.to_owned(),
            cookie: Default::default(),
            timetable: TIMETABLE.to_owned(),
        }
    }
}
