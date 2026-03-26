//! Error types for Bakong KHQR SDK

use thiserror::Error;

#[derive(Debug, Error)]
pub enum BakongError {
    #[error("Invalid QR code format")]
    InvalidQrFormat,

    #[error("Invalid amount for currency: {0}")]
    InvalidAmount(String),

    #[error("Invalid account information: {0}")]
    InvalidAccount(String),

    #[error("Required field missing: {0}")]
    RequiredField(String),

    #[error("Invalid format: {0}")]
    InvalidFormat(String),

    #[error("CRC checksum is invalid (expected: {expected}, actual: {actual})")]
    CrcInvalid { expected: String, actual: String },

    #[error("HTTP error: {0}")]
    HttpError(String),

    #[error("JSON error: {0}")]
    JsonError(String),

    #[error("API error (code: {code}): {message}")]
    ApiError { code: i32, message: String },

    #[error("Request error: {0}")]
    ReqwestError(#[from] reqwest::Error),
}

pub type Result<T> = std::result::Result<T, BakongError>;
