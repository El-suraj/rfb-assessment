use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Configuration Error: {0}")]
    Config(String),

    #[error("HTTP / Network Error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("Bitcoin Core RPC Error (Code {code}): {message}")]
    Rpc { code: i64, message: String },

    #[error("Serialization/Deserialization Error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Command Error: {0}")]
    Command(String),
}

pub type Result<T> = std::result::Result<T, AppError>;