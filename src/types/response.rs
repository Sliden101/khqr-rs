//! Response types for KHQR operations

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QRResult {
    pub qr: String,
    pub md5: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecodedKHQRData {
    pub payload_format_indicator: String,
    pub point_of_initiation_method: Option<String>,
    pub account_information: Option<String>,
    pub merchant_id: Option<String>,
    pub acquiring_bank: Option<String>,
    pub merchant_name: String,
    pub merchant_city: String,
    pub currency: String,
    pub amount: Option<f64>,
    pub country_code: String,
    #[serde(default)]
    pub additional_data: AdditionalData,
    #[serde(default)]
    pub merchant_account_type: String,
    #[serde(default)]
    pub timestamp: Option<TimestampInfo>,
    #[serde(default)]
    pub language: Option<LanguageInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AdditionalData {
    pub bill_number: Option<String>,
    pub mobile_number: Option<String>,
    pub store_label: Option<String>,
    pub terminal_label: Option<String>,
    pub purpose_of_transaction: Option<String>,
    pub merchant_reference_number: Option<String>,
    pub backoffice_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TimestampInfo {
    pub creation_timestamp: Option<i64>,
    pub expiration_timestamp: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LanguageInfo {
    pub language_preference: Option<String>,
    pub merchant_name_alternate_language: Option<String>,
    pub merchant_city_alternate_language: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyResult {
    pub is_valid: bool,
    pub expected_crc: String,
    pub actual_crc: String,
    #[serde(default)]
    pub errors: Vec<String>,
}

impl VerifyResult {
    pub fn valid(expected: String, actual: String) -> Self {
        Self {
            is_valid: expected.to_uppercase() == actual.to_uppercase(),
            expected_crc: expected,
            actual_crc: actual,
            errors: vec![],
        }
    }

    pub fn invalid(expected: String, actual: String, errors: Vec<String>) -> Self {
        Self {
            is_valid: false,
            expected_crc: expected,
            actual_crc: actual,
            errors,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecodedRawField {
    pub tag: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecodedRawData {
    pub fields: Vec<DecodedRawField>,
}
