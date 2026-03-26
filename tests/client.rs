use khqr::{BakongConfig, BakongKHQR, IndividualInfo, MerchantInfo};

#[test]
fn test_new_client() {
    let client = BakongKHQR::new("test_token");
    assert_eq!(client.token(), "test_token");
    assert!(client.is_sandbox());
}

#[test]
fn test_production_client() {
    let config = BakongConfig::production("test_token");
    let client = BakongKHQR::with_config(config);
    assert!(!client.is_sandbox());
}

#[test]
fn test_generate_individual_qr() {
    let client = BakongKHQR::new("test_token");

    let info = IndividualInfo::builder()
        .bakong_account_id("test@bank")
        .merchant_name("Test Shop")
        .merchant_city("Phnom Penh")
        .build()
        .unwrap();

    let result = client.generate_qr(info).unwrap();
    assert!(result.qr.starts_with("000201"));
    assert!(result.qr.contains("63"));
    assert_eq!(result.md5.len(), 32);
}

#[test]
fn test_generate_individual_qr_with_amount() {
    let client = BakongKHQR::new("test_token");

    let info = IndividualInfo::builder()
        .bakong_account_id("test@bank")
        .merchant_name("Test Shop")
        .currency("USD")
        .amount(25.50)
        .build()
        .unwrap();

    let result = client.generate_qr(info).unwrap();
    assert!(result.qr.contains("12")); // Dynamic QR
    assert!(result.qr.contains("54")); // Amount field
}

#[test]
fn test_generate_merchant_qr() {
    let client = BakongKHQR::new("test_token");

    let info = MerchantInfo::builder()
        .bakong_account_id("merchant@bank")
        .merchant_id("MERCHANT123")
        .acquiring_bank("ABA Bank")
        .merchant_name("Big Store")
        .amount(100.00)
        .build()
        .unwrap();

    let result = client.generate_merchant_qr(info).unwrap();
    assert!(result.qr.starts_with("000201"));
    assert!(result.qr.contains("30")); // Merchant account tag
}
