//! Merchant account information for KHQR generation

use serde::{Deserialize, Serialize};

use crate::constants::currency;
use crate::constants::defaults::*;
use crate::constants::limits::*;
use crate::constants::poi;
use crate::error::{BakongError, Result};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MerchantInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bakong_account_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acquiring_bank: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_category_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bill_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminal_label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose_of_transaction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_preference: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_name_alternate_language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_city_alternate_language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_timestamp: Option<i64>,
}

impl MerchantInfo {
    pub fn builder() -> MerchantInfoBuilder {
        MerchantInfoBuilder::new()
    }
}

pub struct MerchantInfoBuilder {
    bakong_account_id: Option<String>,
    merchant_id: Option<String>,
    acquiring_bank: Option<String>,
    merchant_name: Option<String>,
    merchant_city: Option<String>,
    merchant_category_code: Option<String>,
    currency: Option<String>,
    amount: Option<f64>,
    bill_number: Option<String>,
    mobile_number: Option<String>,
    store_label: Option<String>,
    terminal_label: Option<String>,
    purpose_of_transaction: Option<String>,
    language_preference: Option<String>,
    merchant_name_alternate_language: Option<String>,
    merchant_city_alternate_language: Option<String>,
    expiration_timestamp: Option<i64>,
}

impl MerchantInfoBuilder {
    pub fn new() -> Self {
        Self {
            bakong_account_id: None,
            merchant_id: None,
            acquiring_bank: None,
            merchant_name: None,
            merchant_city: None,
            merchant_category_code: None,
            currency: Some(DEFAULT_CURRENCY.to_string()),
            amount: None,
            bill_number: None,
            mobile_number: None,
            store_label: None,
            terminal_label: None,
            purpose_of_transaction: None,
            language_preference: None,
            merchant_name_alternate_language: None,
            merchant_city_alternate_language: None,
            expiration_timestamp: None,
        }
    }

    pub fn bakong_account_id(mut self, id: impl Into<String>) -> Self {
        self.bakong_account_id = Some(id.into());
        self
    }

    pub fn merchant_id(mut self, id: impl Into<String>) -> Self {
        self.merchant_id = Some(id.into());
        self
    }

    pub fn acquiring_bank(mut self, bank: impl Into<String>) -> Self {
        self.acquiring_bank = Some(bank.into());
        self
    }

    pub fn merchant_name(mut self, name: impl Into<String>) -> Self {
        self.merchant_name = Some(name.into());
        self
    }

    pub fn merchant_city(mut self, city: impl Into<String>) -> Self {
        self.merchant_city = Some(city.into());
        self
    }

    pub fn merchant_category_code(mut self, mcc: impl Into<String>) -> Self {
        self.merchant_category_code = Some(mcc.into());
        self
    }

    pub fn currency(mut self, currency: impl Into<String>) -> Self {
        self.currency = Some(currency.into());
        self
    }

    pub fn amount(mut self, amount: f64) -> Self {
        self.amount = Some(amount);
        self
    }

    pub fn bill_number(mut self, bill: impl Into<String>) -> Self {
        self.bill_number = Some(bill.into());
        self
    }

    pub fn mobile_number(mut self, phone: impl Into<String>) -> Self {
        self.mobile_number = Some(phone.into());
        self
    }

    pub fn store_label(mut self, label: impl Into<String>) -> Self {
        self.store_label = Some(label.into());
        self
    }

    pub fn terminal_label(mut self, label: impl Into<String>) -> Self {
        self.terminal_label = Some(label.into());
        self
    }

    pub fn purpose_of_transaction(mut self, purpose: impl Into<String>) -> Self {
        self.purpose_of_transaction = Some(purpose.into());
        self
    }

    pub fn language_preference(mut self, lang: impl Into<String>) -> Self {
        self.language_preference = Some(lang.into());
        self
    }

    pub fn merchant_name_alternate_language(mut self, name: impl Into<String>) -> Self {
        self.merchant_name_alternate_language = Some(name.into());
        self
    }

    pub fn merchant_city_alternate_language(mut self, city: impl Into<String>) -> Self {
        self.merchant_city_alternate_language = Some(city.into());
        self
    }

    pub fn expiration_timestamp(mut self, timestamp: i64) -> Self {
        self.expiration_timestamp = Some(timestamp);
        self
    }

    pub fn build(self) -> Result<MerchantInfo> {
        let bakong_account_id = self
            .bakong_account_id
            .ok_or_else(|| BakongError::RequiredField("bakong_account_id".to_string()))?;

        if bakong_account_id.len() > BAKONG_ACCOUNT_ID_MAX {
            return Err(BakongError::InvalidFormat(format!(
                "bakong_account_id exceeds max length {}",
                BAKONG_ACCOUNT_ID_MAX
            )));
        }

        let merchant_id = self
            .merchant_id
            .ok_or_else(|| BakongError::RequiredField("merchant_id".to_string()))?;

        if merchant_id.len() > MERCHANT_ID_MAX {
            return Err(BakongError::InvalidFormat(format!(
                "merchant_id exceeds max length {}",
                MERCHANT_ID_MAX
            )));
        }

        let acquiring_bank = self
            .acquiring_bank
            .ok_or_else(|| BakongError::RequiredField("acquiring_bank".to_string()))?;

        if acquiring_bank.len() > ACQUIRING_BANK_MAX {
            return Err(BakongError::InvalidFormat(format!(
                "acquiring_bank exceeds max length {}",
                ACQUIRING_BANK_MAX
            )));
        }

        let merchant_name = self
            .merchant_name
            .ok_or_else(|| BakongError::RequiredField("merchant_name".to_string()))?;

        if merchant_name.len() > MERCHANT_NAME_MAX {
            return Err(BakongError::InvalidFormat(format!(
                "merchant_name exceeds max length {}",
                MERCHANT_NAME_MAX
            )));
        }

        let currency = self
            .currency
            .unwrap_or_else(|| DEFAULT_CURRENCY.to_string());
        let _currency_code = currency::code_for(&currency);

        if let Some(amount) = self.amount {
            if currency == "KHR" || currency == "KHR" {
                if amount.fract() != 0.0 {
                    return Err(BakongError::InvalidAmount(
                        "KHR amount must be a whole number".to_string(),
                    ));
                }
            } else if currency == "USD" || currency == "USD" {
                let decimals = (amount.fract() * 100.0).round();
                if decimals > 99.0 {
                    return Err(BakongError::InvalidAmount(
                        "USD amount can have max 2 decimal places".to_string(),
                    ));
                }
            }
        }

        Ok(MerchantInfo {
            bakong_account_id: Some(bakong_account_id),
            merchant_id: Some(merchant_id),
            acquiring_bank: Some(acquiring_bank),
            merchant_name: Some(merchant_name),
            merchant_city: Some(
                self.merchant_city
                    .unwrap_or_else(|| DEFAULT_MERCHANT_CITY.to_string()),
            ),
            merchant_category_code: self.merchant_category_code,
            currency: Some(currency),
            amount: self.amount,
            bill_number: self.bill_number,
            mobile_number: self.mobile_number,
            store_label: self.store_label,
            terminal_label: self.terminal_label,
            purpose_of_transaction: self.purpose_of_transaction,
            language_preference: self.language_preference,
            merchant_name_alternate_language: self.merchant_name_alternate_language,
            merchant_city_alternate_language: self.merchant_city_alternate_language,
            expiration_timestamp: self.expiration_timestamp,
        })
    }
}

impl Default for MerchantInfoBuilder {
    fn default() -> Self {
        Self::new()
    }
}

pub fn is_dynamic(info: &MerchantInfo) -> bool {
    info.amount.map(|a| a > 0.0).unwrap_or(false)
}

pub fn get_poi(info: &MerchantInfo) -> &'static str {
    if is_dynamic(info) {
        poi::DYNAMIC_QR
    } else {
        poi::STATIC_QR
    }
}
