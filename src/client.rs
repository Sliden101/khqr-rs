//! Bakong KHQR SDK
//!
//! Unified SDK for Bakong KHQR (Cambodia's centralized QR payment system).
//! Provides both QR code generation and Bakong API integration.

use std::sync::Arc;

use reqwest::Client;
use serde::de::DeserializeOwned;
use serde::Serialize;
use tracing::{debug, error};

use crate::config::{BakongConfig, Environment};
use crate::crc::calculate_crc16;
use crate::error::{BakongError, Result};
use crate::types::api::*;
use crate::types::individual::IndividualInfo;
use crate::types::merchant::MerchantInfo;
use crate::types::response::QRResult;
use crate::utils::{format_amount, format_sub_tag_length_value, format_tag_length_value, md5_hash};

#[derive(Clone)]
pub struct BakongKHQR {
    config: Arc<BakongConfig>,
    http_client: Client,
}

impl BakongKHQR {
    /// Create a new BakongKHQR instance with the given API token
    pub fn new(token: impl Into<String>) -> Self {
        Self::with_config(BakongConfig::sandbox(token))
    }

    /// Create with custom configuration
    pub fn with_config(config: BakongConfig) -> Self {
        let http_client = Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout_secs))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            config: Arc::new(config),
            http_client,
        }
    }

    /// Get the current token
    pub fn token(&self) -> &str {
        &self.config.token
    }

    /// Get the environment (sandbox or production)
    pub fn environment(&self) -> Environment {
        self.config.environment
    }

    /// Get the base URL
    pub fn base_url(&self) -> &str {
        self.config
            .base_url
            .as_deref()
            .unwrap_or_else(|| self.config.environment.base_url())
    }

    /// Check if using sandbox environment
    pub fn is_sandbox(&self) -> bool {
        self.config.is_sandbox()
    }

    /// Generate QR code for individual account
    pub fn generate_qr(&self, info: IndividualInfo) -> Result<QRResult> {
        self.generate_individual(&info)
    }

    /// Generate QR code for merchant account
    pub fn generate_merchant_qr(&self, info: MerchantInfo) -> Result<QRResult> {
        self.generate_merchant_internal(&info)
    }

    fn generate_individual(&self, info: &IndividualInfo) -> Result<QRResult> {
        let bakong_account_id = info
            .bakong_account_id
            .as_ref()
            .ok_or_else(|| BakongError::RequiredField("bakong_account_id".to_string()))?;

        let merchant_name = info
            .merchant_name
            .as_ref()
            .ok_or_else(|| BakongError::RequiredField("merchant_name".to_string()))?;

        let merchant_city = info
            .merchant_city
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or("Phnom Penh");

        let currency = info.currency.as_ref().map(|s| s.as_str()).unwrap_or("KHR");

        let is_dynamic = info.amount.map(|a| a > 0.0).unwrap_or(false);
        let poi = if is_dynamic { "12" } else { "11" };

        let mut qr_parts = Vec::new();

        qr_parts.push(format_tag_length_value("00", "01"));
        qr_parts.push(format_tag_length_value("01", poi));

        let account_info =
            Self::build_individual_account_info(bakong_account_id, &info.account_information);
        qr_parts.push(format_tag_length_value("29", &account_info));

        qr_parts.push(format_tag_length_value("52", "0000"));

        let currency_code = if currency.to_uppercase() == "KHR" {
            "116"
        } else {
            "840"
        };
        qr_parts.push(format_tag_length_value("53", currency_code));

        if is_dynamic {
            if let Some(amount) = info.amount {
                let amount_str = format_amount(amount, currency);
                qr_parts.push(format_tag_length_value("54", &amount_str));
            }
        }

        qr_parts.push(format_tag_length_value("58", "KH"));
        qr_parts.push(format_tag_length_value("59", merchant_name));
        qr_parts.push(format_tag_length_value("60", merchant_city));

        let additional_data = Self::build_additional_data_individual(info);
        if !additional_data.is_empty() {
            qr_parts.push(format_tag_length_value("62", &additional_data));
        }

        let payload = qr_parts.join("");
        let crc = calculate_crc16(&payload);
        let qr_string = format!("{}63{}", payload, crc);

        let md5 = md5_hash(&qr_string);

        Ok(QRResult { qr: qr_string, md5 })
    }

    fn generate_merchant_internal(&self, info: &MerchantInfo) -> Result<QRResult> {
        let bakong_account_id = info
            .bakong_account_id
            .as_ref()
            .ok_or_else(|| BakongError::RequiredField("bakong_account_id".to_string()))?;

        let merchant_id = info
            .merchant_id
            .as_ref()
            .ok_or_else(|| BakongError::RequiredField("merchant_id".to_string()))?;

        let acquiring_bank = info
            .acquiring_bank
            .as_ref()
            .ok_or_else(|| BakongError::RequiredField("acquiring_bank".to_string()))?;

        let merchant_name = info
            .merchant_name
            .as_ref()
            .ok_or_else(|| BakongError::RequiredField("merchant_name".to_string()))?;

        let merchant_city = info
            .merchant_city
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or("Phnom Penh");

        let currency = info.currency.as_ref().map(|s| s.as_str()).unwrap_or("KHR");

        let is_dynamic = info.amount.map(|a| a > 0.0).unwrap_or(false);
        let poi = if is_dynamic { "12" } else { "11" };

        let mut qr_parts = Vec::new();

        qr_parts.push(format_tag_length_value("00", "01"));
        qr_parts.push(format_tag_length_value("01", poi));

        let account_info =
            Self::build_merchant_account_info(bakong_account_id, merchant_id, acquiring_bank);
        qr_parts.push(format_tag_length_value("30", &account_info));

        if let Some(mcc) = &info.merchant_category_code {
            qr_parts.push(format_tag_length_value("52", mcc));
        } else {
            qr_parts.push(format_tag_length_value("52", "0000"));
        }

        let currency_code = if currency.to_uppercase() == "KHR" {
            "116"
        } else {
            "840"
        };
        qr_parts.push(format_tag_length_value("53", currency_code));

        if is_dynamic {
            if let Some(amount) = info.amount {
                let amount_str = format_amount(amount, currency);
                qr_parts.push(format_tag_length_value("54", &amount_str));
            }
        }

        qr_parts.push(format_tag_length_value("58", "KH"));
        qr_parts.push(format_tag_length_value("59", merchant_name));
        qr_parts.push(format_tag_length_value("60", merchant_city));

        let additional_data = Self::build_additional_data_merchant(info);
        if !additional_data.is_empty() {
            qr_parts.push(format_tag_length_value("62", &additional_data));
        }

        let payload = qr_parts.join("");
        let crc = calculate_crc16(&payload);
        let qr_string = format!("{}63{}", payload, crc);

        let md5 = md5_hash(&qr_string);

        Ok(QRResult { qr: qr_string, md5 })
    }

    fn build_individual_account_info(bakong_id: &str, account_info: &Option<String>) -> String {
        let mut parts = Vec::new();
        parts.push(format_sub_tag_length_value("29", "00", bakong_id));

        if let Some(info) = account_info {
            parts.push(format_sub_tag_length_value("29", "01", info));
        }

        parts.join("")
    }

    fn build_merchant_account_info(
        bakong_id: &str,
        merchant_id: &str,
        acquiring_bank: &str,
    ) -> String {
        let mut parts = Vec::new();
        parts.push(format_sub_tag_length_value("30", "00", bakong_id));
        parts.push(format_sub_tag_length_value("30", "01", merchant_id));
        parts.push(format_sub_tag_length_value("30", "02", acquiring_bank));
        parts.join("")
    }

    fn build_additional_data_individual(info: &IndividualInfo) -> String {
        let mut parts = Vec::new();

        if let Some(bill) = &info.bill_number {
            parts.push(format_sub_tag_length_value("62", "01", bill));
        }
        if let Some(mobile) = &info.mobile_number {
            parts.push(format_sub_tag_length_value("62", "02", mobile));
        }
        if let Some(store) = &info.store_label {
            parts.push(format_sub_tag_length_value("62", "03", store));
        }
        if let Some(terminal) = &info.terminal_label {
            parts.push(format_sub_tag_length_value("62", "04", terminal));
        }
        if let Some(purpose) = &info.purpose_of_transaction {
            parts.push(format_sub_tag_length_value("62", "05", purpose));
        }

        let mut timestamp_parts = Vec::new();
        let now = chrono::Utc::now().timestamp_millis();
        timestamp_parts.push(format_sub_tag_length_value("62", "00", &now.to_string()));

        if let Some(expire) = info.expiration_timestamp {
            timestamp_parts.push(format_sub_tag_length_value("62", "01", &expire.to_string()));
        }

        if !timestamp_parts.is_empty() {
            parts.push(format_tag_length_value("68", &timestamp_parts.join("")));
        }

        parts.join("")
    }

    fn build_additional_data_merchant(info: &MerchantInfo) -> String {
        let mut parts = Vec::new();

        if let Some(bill) = &info.bill_number {
            parts.push(format_sub_tag_length_value("62", "01", bill));
        }
        if let Some(mobile) = &info.mobile_number {
            parts.push(format_sub_tag_length_value("62", "02", mobile));
        }
        if let Some(store) = &info.store_label {
            parts.push(format_sub_tag_length_value("62", "03", store));
        }
        if let Some(terminal) = &info.terminal_label {
            parts.push(format_sub_tag_length_value("62", "04", terminal));
        }
        if let Some(purpose) = &info.purpose_of_transaction {
            parts.push(format_sub_tag_length_value("62", "05", purpose));
        }

        let mut timestamp_parts = Vec::new();
        let now = chrono::Utc::now().timestamp_millis();
        timestamp_parts.push(format_sub_tag_length_value("62", "00", &now.to_string()));

        if let Some(expire) = info.expiration_timestamp {
            timestamp_parts.push(format_sub_tag_length_value("62", "01", &expire.to_string()));
        }

        if !timestamp_parts.is_empty() {
            parts.push(format_tag_length_value("68", &timestamp_parts.join("")));
        }

        parts.join("")
    }

    // ==================== API Methods ====================

    /// Check if a Bakong account exists
    pub async fn check_bakong_account(
        &self,
        account_id: &str,
    ) -> Result<CheckBakongAccountResponse> {
        let request = CheckBakongAccountRequest {
            account_id: account_id.to_string(),
        };

        self.post("/v1/check_bakong_account", &request).await
    }

    /// Check transaction status by MD5 hash
    pub async fn check_transaction_by_md5(
        &self,
        md5: &str,
    ) -> Result<CheckTransactionByMd5Response> {
        let request = CheckTransactionByMd5Request {
            md5: md5.to_string(),
        };

        self.post("/v1/check_transaction_by_md5", &request).await
    }

    /// Check transaction status by hash
    pub async fn check_transaction_by_hash(
        &self,
        hash: &str,
    ) -> Result<CheckTransactionByMd5Response> {
        #[derive(Serialize)]
        struct Request<'a> {
            hash: &'a str,
        }

        let request = Request { hash };
        self.post("/v1/check_transaction_by_hash", &request).await
    }

    /// Check transaction status by short hash
    pub async fn check_transaction_by_short_hash(
        &self,
        short_hash: &str,
    ) -> Result<CheckTransactionByMd5Response> {
        #[derive(Serialize)]
        struct Request<'a> {
            #[serde(rename = "shortHash")]
            short_hash: &'a str,
        }

        let request = Request { short_hash };
        self.post("/v1/check_transaction_by_short_hash", &request)
            .await
    }

    /// Generate deeplink from QR code
    pub async fn generate_deeplink(
        &self,
        qr: &str,
        source_info: SourceInfo,
    ) -> Result<GenerateDeeplinkResponse> {
        let request = GenerateDeeplinkRequest {
            qr: qr.to_string(),
            source_info,
        };

        self.post("/v1/generate_deeplink_by_qr", &request).await
    }

    /// Refresh/renew API token
    pub async fn refresh_token(&self, current_token: &str) -> Result<TokenRefreshResponse> {
        let request = TokenRefreshRequest {
            token: current_token.to_string(),
        };

        self.post("/v1/renew_token", &request).await
    }

    async fn post<T: Serialize + ?Sized, R: DeserializeOwned>(
        &self,
        endpoint: &str,
        body: &T,
    ) -> Result<R> {
        let base = self.base_url();
        let url = if base.ends_with('/') {
            format!("{}{}", base, endpoint)
        } else {
            format!("{}/{}", base, endpoint)
        };

        debug!("POST {}", url);

        let response = self
            .http_client
            .post(&url)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.config.token))
            .json(body)
            .send()
            .await?;

        let status = response.status();
        let response_text = response.text().await?;

        debug!("Response status: {}, body: {}", status, response_text);

        if !status.is_success() {
            error!("HTTP error: {} - {}", status, response_text);
            return Err(BakongError::HttpError(format!(
                "{}: {}",
                status, response_text
            )));
        }

        serde_json::from_str(&response_text).map_err(|e| {
            error!("JSON parse error: {}", e);
            BakongError::JsonError(e.to_string())
        })
    }
}

impl std::fmt::Debug for BakongKHQR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BakongKHQR")
            .field("environment", &self.config.environment)
            .finish()
    }
}
