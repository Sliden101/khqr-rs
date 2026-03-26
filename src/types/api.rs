//! API types for Bakong API client

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckBakongAccountRequest {
    pub account_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckBakongAccountResponse {
    pub status: ApiResponseStatus,
    #[serde(default)]
    pub data: Option<CheckBakongAccountData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckBakongAccountData {
    #[serde(rename = "bakongAccountExisted")]
    pub bakong_account_existed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckTransactionByMd5Request {
    pub md5: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckTransactionByMd5Response {
    pub status: ApiResponseStatus,
    #[serde(default)]
    pub data: Option<CheckTransactionData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckTransactionData {
    #[serde(rename = "transactionId")]
    pub transaction_id: Option<String>,
    #[serde(rename = "sourceAccount")]
    pub source_account: Option<String>,
    #[serde(rename = "destinationAccount")]
    pub destination_account: Option<String>,
    #[serde(rename = "amount")]
    pub amount: Option<String>,
    #[serde(rename = "currency")]
    pub currency: Option<String>,
    #[serde(rename = "status")]
    pub status: Option<String>,
    #[serde(rename = "transactionDate")]
    pub transaction_date: Option<String>,
    #[serde(rename = "refunded")]
    pub refunded: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateDeeplinkRequest {
    pub qr: String,
    pub source_info: SourceInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceInfo {
    #[serde(rename = "appIconUrl")]
    pub app_icon_url: Option<String>,
    #[serde(rename = "appName")]
    pub app_name: String,
    #[serde(rename = "appDeepLinkCallback")]
    pub app_deep_link_callback: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateDeeplinkResponse {
    pub status: ApiResponseStatus,
    #[serde(default)]
    pub data: Option<GenerateDeeplinkData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateDeeplinkData {
    #[serde(rename = "deeplinkUrl")]
    pub deeplink_url: Option<String>,
    #[serde(rename = "qrImage")]
    pub qr_image: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponseStatus {
    pub code: i32,
    pub message: Option<String>,
}

impl ApiResponseStatus {
    pub fn success() -> Self {
        Self {
            code: 0,
            message: Some("Success".to_string()),
        }
    }

    pub fn is_success(&self) -> bool {
        self.code == 0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenRefreshRequest {
    pub token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenRefreshResponse {
    pub status: ApiResponseStatus,
    #[serde(default)]
    pub data: Option<TokenRefreshData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenRefreshData {
    #[serde(rename = "accessToken")]
    pub access_token: String,
    #[serde(rename = "tokenType")]
    pub token_type: String,
    #[serde(rename = "expiresIn")]
    pub expires_in: i64,
}
