pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Config file not found")]
    ConfigFileNotFound,

    #[error("Failed to open browser")]
    OpenBrowserFailed(#[from] opener::OpenError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Failed to load : {0}")]
    TomlParse(String),

    #[error("Failed to save config file : {0}")]
    SaveConfigFile(std::io::Error),

    #[error("Failed to load Cookie: {0}")]
    TokioTaskJoin(#[from] tokio::task::JoinError),

    #[error("Manaba error occured")]
    Manaba(#[from] manaba_sdk::error::ManabaError),
}
