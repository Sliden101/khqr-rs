//! Utility functions for Bakong KHQR SDK

use md5::{Digest, Md5};

pub fn md5_hash(data: &str) -> String {
    let mut hasher = Md5::new();
    hasher.update(data.as_bytes());
    format!("{:x}", hasher.finalize())
}

pub fn format_amount(amount: f64, currency: &str) -> String {
    match currency.to_uppercase().as_str() {
        "KHR" => format!("{:.0}", amount),
        _ => format!("{:.2}", amount),
    }
}

pub fn pad_length(value: &str) -> String {
    format!("{:02}", value.len())
}

pub fn format_tag_length_value(tag: &str, value: &str) -> String {
    format!("{}{}{}", tag, pad_length(value), value)
}

pub fn parse_tag_length_value(data: &str) -> Option<(&str, &str)> {
    if data.len() < 4 {
        return None;
    }
    let tag = &data[..2];
    let length: usize = data[2..4].parse().ok()?;
    if data.len() < 4 + length {
        return None;
    }
    let value = &data[4..4 + length];
    Some((tag, value))
}

pub fn format_sub_tag_length_value(_main_tag: &str, sub_tag: &str, value: &str) -> String {
    format!("{}{}{}", sub_tag, pad_length(value), value)
}
