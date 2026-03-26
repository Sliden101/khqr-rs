//! KHQR Generator - generates EMV-compliant QR codes for Bakong

use crate::crc::calculate_crc16;
use crate::error::{BakongError, Result};
use crate::types::individual::{self, IndividualInfo};
use crate::types::merchant::{self, MerchantInfo};
use crate::types::response::QRResult;
use crate::utils::{format_amount, format_sub_tag_length_value, format_tag_length_value, md5_hash};

pub struct KHQR;

impl KHQR {
    pub fn generate(info: IndividualInfo) -> Result<QRResult> {
        Self::generate_individual(&info)
    }

    pub fn generate_merchant(info: MerchantInfo) -> Result<QRResult> {
        Self::generate_merchant_internal(&info)
    }

    fn generate_individual(info: &IndividualInfo) -> Result<QRResult> {
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

        let is_dynamic = individual::is_dynamic(info);
        let poi = individual::get_poi(info);

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

        let additional_data = Self::build_additional_data(info);
        if !additional_data.is_empty() {
            qr_parts.push(format_tag_length_value("62", &additional_data));
        }

        let payload = qr_parts.join("");
        let crc = calculate_crc16(&payload);
        let qr_string = format!("{}63{}", payload, crc);

        let md5 = md5_hash(&qr_string);

        Ok(QRResult { qr: qr_string, md5 })
    }

    fn generate_merchant_internal(info: &MerchantInfo) -> Result<QRResult> {
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

        let is_dynamic = merchant::is_dynamic(info);
        let poi = merchant::get_poi(info);

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

    fn build_additional_data(info: &IndividualInfo) -> String {
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
}
