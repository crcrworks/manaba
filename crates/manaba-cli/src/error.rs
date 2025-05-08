pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Config file not found")]
    ConfigFileNotFound,

    #[error("{0}")]
    OpenBrowserFailed(#[from] opener::OpenError),

    #[error("{0}")]
    Io(#[from] std::io::Error),

    #[error("Failed to load : {0}")]
    TomlParse(String),

    #[error("Failed to save config file: {0}")]
    SaveConfigFile(std::io::Error),

    #[error("{0}")]
    TokioTaskJoin(#[from] tokio::task::JoinError),

    #[error("Failed to manage config file: {0}")]
    ManageConfig(String),

    #[error("{0}")]
    Manaba(#[from] manaba_sdk::error::ManabaError),
}
