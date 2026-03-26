//! Constants for Bakong KHQR SDK

//! EMV QR Code Tags
pub mod emv_tags {
    pub const PAYLOAD_FORMAT_INDICATOR: &str = "00";
    pub const POINT_OF_INITIATION_METHOD: &str = "01";
    pub const MERCHANT_ACCOUNT_INFORMATION_INDIVIDUAL: &str = "29";
    pub const MERCHANT_ACCOUNT_INFORMATION_MERCHANT: &str = "30";
    pub const MERCHANT_CATEGORY_CODE: &str = "52";
    pub const TRANSACTION_CURRENCY: &str = "53";
    pub const TRANSACTION_AMOUNT: &str = "54";
    pub const TIP_OR_CONVENIENCE_FEE: &str = "55";
    pub const COUNTRY_CODE: &str = "58";
    pub const MERCHANT_NAME: &str = "59";
    pub const MERCHANT_CITY: &str = "60";
    pub const ADDITIONAL_DATA_FIELD_TEMPLATE: &str = "62";
    pub const CRC: &str = "63";

    pub mod merchant_account {
        pub const BAKONG_ACCOUNT_IDENTIFIER: &str = "00";
        pub const ACQUIRING_BANK: &str = "02";
    }

    pub mod additional_data {
        pub const BILL_NUMBER: &str = "01";
        pub const MOBILE_NUMBER: &str = "02";
        pub const STORE_LABEL: &str = "03";
        pub const TERMINAL_LABEL: &str = "04";
        pub const PURPOSE_OF_TRANSACTION: &str = "05";
        pub const MERCHANT_REFERENCE_NUMBER: &str = "01";
        pub const BACKOFFICE_URL: &str = "68";
    }

    pub mod timestamp {
        pub const CREATION_TIMESTAMP: &str = "00";
        pub const EXPIRATION_TIMESTAMP: &str = "01";
    }

    pub mod language {
        pub const LANGUAGE_PREFERENCE: &str = "00";
        pub const MERCHANT_NAME_ALTERNATE: &str = "01";
        pub const MERCHANT_CITY_ALTERNATE: &str = "02";
    }
}

/// Currency codes (ISO 4217)
pub mod currency {
    pub const KHR: &str = "116";
    pub const USD: &str = "840";

    pub fn code_for(currency: &str) -> &str {
        match currency.to_uppercase().as_str() {
            "KHR" => KHR,
            "USD" => USD,
            _ => USD,
        }
    }
}

/// Point of initiation method
pub mod poi {
    pub const STATIC_QR: &str = "11";
    pub const DYNAMIC_QR: &str = "12";
}

/// Default values
pub mod defaults {
    pub const MERCHANT_CATEGORY_CODE: &str = "0000";
    pub const COUNTRY_CODE: &str = "KH";
    pub const DEFAULT_MERCHANT_CITY: &str = "Phnom Penh";
    pub const DEFAULT_CURRENCY: &str = "KHR";
    pub const CRC_LENGTH: usize = 4;
}

/// Field length limits
pub mod limits {
    pub const BAKONG_ACCOUNT_ID_MAX: usize = 32;
    pub const MERCHANT_NAME_MAX: usize = 25;
    pub const MERCHANT_CITY_MAX: usize = 15;
    pub const ACCOUNT_INFORMATION_MAX: usize = 32;
    pub const ACQUIRING_BANK_MAX: usize = 32;
    pub const MERCHANT_ID_MAX: usize = 32;
    pub const BILL_NUMBER_MAX: usize = 25;
    pub const MOBILE_NUMBER_MAX: usize = 25;
    pub const STORE_LABEL_MAX: usize = 25;
    pub const TERMINAL_LABEL_MAX: usize = 25;
    pub const PURPOSE_MAX: usize = 25;
    pub const MERCHANT_NAME_ALTERNATE_MAX: usize = 25;
    pub const MERCHANT_CITY_ALTERNATE_MAX: usize = 15;
    pub const LANGUAGE_PREFERENCE_MAX: usize = 2;
}
