//! # Bakong KHQR SDK
//!
//! Unofficial Rust SDK for Bakong KHQR (Cambodia's centralized QR payment system).
//!
//! ## Features
//!
//! - QR Code Generation (Individual and Merchant accounts)
//! - QR Code Decoding
//! - CRC16 Verification
//! - Bakong API Integration
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use bakong_khqr::{BakongKHQR, IndividualInfo};
//!
//! let khqr = BakongKHQR::new("your_token");
//!
//! // Generate QR code
//! let info = IndividualInfo::builder()
//!     .bakong_account_id("user@bank")
//!     .merchant_name("Coffee Shop")
//!     .merchant_city("Phnom Penh")
//!     .currency("KHR")
//!     .amount(50000.0)
//!     .build()
//!     .unwrap();
//!
//! let result = khqr.generate_qr(info).unwrap();
//! println!("QR Code: {}", result.qr);
//! println!("MD5 Hash: {}", result.md5);
//! ```
//!
//! ## API Usage
//!
//! ```rust,no_run
//! use bakong_khqr::{BakongKHQR, SourceInfo};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), bakong_khqr::BakongError> {
//!     let khqr = BakongKHQR::new("your_token");
//!
//!     // Check if Bakong account exists
//!     let response = khqr.check_bakong_account("user@bank").await?;
//!     println!("Account exists: {:?}", response.data);
//!
//!     // Check transaction by MD5
//!     let tx = khqr.check_transaction_by_md5("md5_hash").await?;
//!
//!     // Generate deeplink
//!     let deeplink = khqr.generate_deeplink(
//!         "qr_string",
//!         SourceInfo {
//!             app_name: "My App".to_string(),
//!             app_icon_url: None,
//!             app_deep_link_callback: None,
//!         },
//!     ).await?;
//!
//!     Ok(())
//! }
//! ```

pub mod client;
pub mod config;
pub mod constants;
pub mod crc;
pub mod decoder;
pub mod error;
pub mod types;
pub mod utils;

pub use client::BakongKHQR;
pub use config::{BakongConfig, Environment};
pub use crc::{calculate_crc16, verify_crc};
pub use decoder::KHQRDecoder;
pub use error::{BakongError, Result};
pub use types::*;
pub use utils::{
    format_amount, format_sub_tag_length_value, format_tag_length_value, md5_hash, pad_length,
    parse_tag_length_value,
};
