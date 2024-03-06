use std::fmt::Debug;

#[derive(thiserror::Error, Debug)]
pub enum FinanceapiError {
    /// Error from `reqwest`
    #[error(transparent)]
    RequestError(#[from] reqwest::Error),
    /// Error while parsing the URL
    #[error("error while parsing URL")]
    UrlParseError(#[from] url::ParseError),
    /// Unable to parse JSON response
    #[error("unable to parse JSON response")]
    JsonParseError,
    /// JSON serde error
    #[error("serde JSON error")]
    JsonSerdeError(#[from] serde_json::Error),
    /// JSON response error
    #[error("JSON response error ({0})")]
    JsonResponseError(String),
    /// Symbol not found
    #[error("symbol not found")]
    SymbolNotFoundError,
    /// Unexpected error
    #[error("unexpected error")]
    UnexpectedError,
}
