use thiserror::Error;

#[derive(Error, Debug)]
pub enum DqrError {
    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("CSV parsing error: {0}")]
    CsvError(#[from] csv::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Validation rule not found: {0}")]
    RuleNotFound(String),

    #[error("Invalid rule configuration: {0}")]
    InvalidRuleConfig(String),

    #[error("JSON path error: {0}")]
    JsonPathError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),
}