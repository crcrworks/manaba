use colored_text::Colorize as _;
use std::{fmt::Display, path::PathBuf};

pub type Result<T, E = Error> = core::result::Result<T, E>;

pub fn print_err<T: AsRef<str> + Display>(e: T) {
    eprintln!("{}", e.red());
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    OpenBrowserFailed(#[from] opener::OpenError),

    #[error("{0}")]
    Io(#[from] std::io::Error),

    #[error(
        "Failed to load config: {source}.\n use default value for now but check config file: {config_path}"
    )]
    LoadConfig {
        source: config::ConfigError,
        config_path: PathBuf,
    },

    #[error("{0}")]
    TokioTaskJoin(#[from] tokio::task::JoinError),

    #[error("{0}")]
    Manaba(#[from] manaba_sdk::error::ManabaError),
}
