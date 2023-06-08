use thiserror::Error;

#[derive(Debug, Error)]
#[error("Failed to generate quote")]
pub struct QuoteError;
