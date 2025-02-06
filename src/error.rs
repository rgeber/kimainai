use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {

    #[error("Reqwest Error.")]
    ReqwestError {
        #[from]
        source: reqwest::Error,
    },

    #[error("An unknown error occured.")]
    UnknownError,
}
