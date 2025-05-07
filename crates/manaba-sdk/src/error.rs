pub type Result<T, E = ManabaError> = core::result::Result<T, E>;

#[derive(thiserror::Error, Debug)]
pub enum ManabaError {
    #[error("Failed to parse HTML body: {0}")]
    SendRequestError(reqwest::Error),

    #[error("Failed to parse HTML body{0}")]
    HtmlBodyParseError(reqwest::Error),

    #[error("Cookie is invalid")]
    InvalidCookie,

    #[error("Failed to load Cookie: {0}")]
    LoadCookie(String),
}
