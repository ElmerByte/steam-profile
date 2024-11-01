use thiserror::Error;
#[derive(Error, Debug)]
pub enum ProfileError {
    #[error("Serialization failed due to invalid JSON format.")]
    SerdeJsonError(#[from] serde_json::Error),

    #[error("An unexpected error occurred: {0}")]
    Anyhow(#[from] anyhow::Error),

    #[error("Standard IO operation failed: {0}")]
    Std(#[from] std::io::Error),

    #[error("Scraper Failed {0}")]
    ScraperError(#[from] scraper::error::SelectorErrorKind<'static>),

    #[error("Fetching Link failed{0}")]
    FetchError(String),

    #[error("Regex Error {0}")]
    RegexError(#[from] regex::Error),
}
