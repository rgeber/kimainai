use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {

    #[error("Reqwest Error.")]
    ReqwestError {
        #[from]
        source: reqwest::Error,
    },

    #[error("IO Error.")]
    StdIoError {
        #[from]
        source: std::io::Error,
    },

    #[error("Toml Error.")]
    TomlError {
        #[from]
        source: toml::de::Error,
    },

    #[error("Reqwest Invalid Header Value.")]
    ReqwestInvalidHeaderValueError {
        #[from]
        source: reqwest::header::InvalidHeaderValue,
    },

    #[error("Serde JSON error")]
    SersdeJsonError {
        #[from]
        source: serde_json::Error,
    },

    #[error("An unknown error occured.")]
    UnknownError,
}
