//! KHQR Decoder - parses EMV QR codes for Bakong

use crate::crc::verify_crc;
use crate::error::{BakongError, Result};
use crate::types::response::{DecodedKHQRData, DecodedRawData, DecodedRawField, VerifyResult};
use crate::utils::parse_tag_length_value;

pub struct KHQRDecoder;

impl KHQRDecoder {
    pub fn decode(qr_string: &str) -> Result<DecodedKHQRData> {
        let (is_valid, expected, actual) = verify_crc(qr_string);
        if !is_valid {
            return Err(BakongError::CrcInvalid { expected, actual });
        }

        let data = qr_string.trim_end_matches(|c: char| c.is_ascii_hexdigit());

        let mut cursor = 0;
        let bytes = data.as_bytes();

        let mut payload_format_indicator = String::new();
        let mut point_of_initiation_method: Option<String> = None;
        let mut account_information: Option<String> = None;
        let mut merchant_id: Option<String> = None;
        let mut acquiring_bank: Option<String> = None;
        let mut merchant_name = String::new();
        let mut merchant_city = String::new();
        let mut currency = String::new();
        let mut amount: Option<f64> = None;
        let mut country_code = String::new();
        let mut account_type = String::new();

        let mut bill_number: Option<String> = None;
        let mut mobile_number: Option<String> = None;
        let mut store_label: Option<String> = None;
        let mut terminal_label: Option<String> = None;
        let mut purpose_of_transaction: Option<String> = None;
        let mut merchant_reference_number: Option<String> = None;
        let mut backoffice_url: Option<String> = None;

        let mut creation_timestamp: Option<i64> = None;
        let mut expiration_timestamp: Option<i64> = None;

        let language_preference: Option<String> = None;
        let merchant_name_alternate: Option<String> = None;
        let merchant_city_alternate: Option<String> = None;

        while cursor < bytes.len() {
            let remaining = &data[cursor..];
            let (tag, value) = match parse_tag_length_value(remaining) {
                Some(v) => v,
                None => break,
            };

            match tag {
                "00" => payload_format_indicator = value.to_string(),
                "01" => point_of_initiation_method = Some(value.to_string()),
                "29" | "30" => {
                    account_type = tag.to_string();
                    account_information = Some(Self::parse_account_info(
                        value,
                        &mut merchant_id,
                        &mut acquiring_bank,
                    ));
                }
                "52" => {} // Merchant category code - not storing
                "53" => currency = value.to_string(),
                "54" => {
                    amount = value.parse().ok();
                }
                "58" => country_code = value.to_string(),
                "59" => merchant_name = value.to_string(),
                "60" => merchant_city = value.to_string(),
                "62" => {
                    Self::parse_additional_data(
                        value,
                        &mut bill_number,
                        &mut mobile_number,
                        &mut store_label,
                        &mut terminal_label,
                        &mut purpose_of_transaction,
                        &mut merchant_reference_number,
                        &mut backoffice_url,
                        &mut creation_timestamp,
                        &mut expiration_timestamp,
                    );
                }
                "63" => break, // CRC - end of data
                _ => {}
            }

            cursor += 2 + 2 + value.len();
        }

        if payload_format_indicator.is_empty() {
            return Err(BakongError::InvalidQrFormat);
        }

        if merchant_name.is_empty() {
            return Err(BakongError::InvalidQrFormat);
        }

        Ok(DecodedKHQRData {
            payload_format_indicator,
            point_of_initiation_method,
            account_information,
            merchant_id,
            acquiring_bank,
            merchant_name,
            merchant_city,
            currency,
            amount,
            country_code,
            additional_data: crate::types::response::AdditionalData {
                bill_number,
                mobile_number,
                store_label,
                terminal_label,
                purpose_of_transaction,
                merchant_reference_number,
                backoffice_url,
            },
            merchant_account_type: account_type,
            timestamp: if creation_timestamp.is_some() || expiration_timestamp.is_some() {
                Some(crate::types::response::TimestampInfo {
                    creation_timestamp,
                    expiration_timestamp,
                })
            } else {
                None
            },
            language: if language_preference.is_some()
                || merchant_name_alternate.is_some()
                || merchant_city_alternate.is_some()
            {
                Some(crate::types::response::LanguageInfo {
                    language_preference,
                    merchant_name_alternate_language: merchant_name_alternate,
                    merchant_city_alternate_language: merchant_city_alternate,
                })
            } else {
                None
            },
        })
    }

    fn parse_account_info(
        data: &str,
        merchant_id: &mut Option<String>,
        acquiring_bank: &mut Option<String>,
    ) -> String {
        let mut cursor = 0;
        let mut bakong_id = String::new();

        while cursor < data.len() {
            let remaining = &data[cursor..];
            let (tag, value) = match parse_tag_length_value(remaining) {
                Some(v) => v,
                None => break,
            };

            match tag {
                "00" => bakong_id = value.to_string(),
                "01" => *merchant_id = Some(value.to_string()),
                "02" => *acquiring_bank = Some(value.to_string()),
                _ => {}
            }

            cursor += 2 + 2 + value.len();
        }

        bakong_id
    }

    fn parse_additional_data(
        data: &str,
        bill_number: &mut Option<String>,
        mobile_number: &mut Option<String>,
        store_label: &mut Option<String>,
        terminal_label: &mut Option<String>,
        purpose_of_transaction: &mut Option<String>,
        merchant_reference_number: &mut Option<String>,
        backoffice_url: &mut Option<String>,
        creation_timestamp: &mut Option<i64>,
        expiration_timestamp: &mut Option<i64>,
    ) {
        let mut cursor = 0;

        // Check for timestamp sub-tag (tag "68" contains timestamp info)
        if data.starts_with("68") {
            if let Some((_, timestamp_data)) = parse_tag_length_value(data) {
                let mut ts_cursor = 0;
                while ts_cursor < timestamp_data.len() {
                    let remaining = &timestamp_data[ts_cursor..];
                    let (tag, value) = match parse_tag_length_value(remaining) {
                        Some(v) => v,
                        None => break,
                    };

                    match tag {
                        "00" => *creation_timestamp = value.parse().ok(),
                        "01" => *expiration_timestamp = value.parse().ok(),
                        _ => {}
                    }

                    ts_cursor += 2 + 2 + value.len();
                }
                return;
            }
        }

        while cursor < data.len() {
            let remaining = &data[cursor..];
            let (tag, value) = match parse_tag_length_value(remaining) {
                Some(v) => v,
                None => break,
            };

            match tag {
                "01" => {
                    if data.starts_with("68") {
                        *merchant_reference_number = Some(value.to_string());
                    } else {
                        *bill_number = Some(value.to_string());
                    }
                }
                "02" => *mobile_number = Some(value.to_string()),
                "03" => *store_label = Some(value.to_string()),
                "04" => *terminal_label = Some(value.to_string()),
                "05" => *purpose_of_transaction = Some(value.to_string()),
                "68" => *backoffice_url = Some(value.to_string()),
                _ => {}
            }

            cursor += 2 + 2 + value.len();
        }
    }

    pub fn decode_raw(qr_string: &str) -> Result<DecodedRawData> {
        let data = qr_string.trim_end_matches(|c: char| c.is_ascii_hexdigit());
        let mut fields = Vec::new();
        let mut cursor = 0;

        while cursor < data.len() {
            let remaining = &data[cursor..];
            let (tag, value) = match parse_tag_length_value(remaining) {
                Some(v) => v,
                None => break,
            };

            fields.push(DecodedRawField {
                tag: tag.to_string(),
                value: value.to_string(),
            });

            cursor += 2 + 2 + value.len();
        }

        Ok(DecodedRawData { fields })
    }

    pub fn verify(qr_string: &str) -> Result<VerifyResult> {
        let (is_valid, expected, actual) = verify_crc(qr_string);

        if is_valid {
            Ok(VerifyResult::valid(expected, actual))
        } else {
            Ok(VerifyResult::invalid(
                expected,
                actual,
                vec!["CRC checksum mismatch".to_string()],
            ))
        }
    }
}
