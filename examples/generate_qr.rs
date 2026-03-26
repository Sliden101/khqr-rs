//! QR Code Generation Example
//!
//! This example demonstrates how to generate KHQR codes for Bakong payment system.
//!
//! Run with: cargo run --example generate_qr --features rustls-tls

use bakong_khqr::{BakongKHQR, IndividualInfo, MerchantInfo};

fn main() {
    println!("=== KHQR Code Generation Example ===\n");

    // Create SDK instance (token required for API calls, but local generation works)
    let khqr = BakongKHQR::new("token");

    // --------------------------------------------------
    // 1. Generate Individual QR (Static - no amount)
    // --------------------------------------------------
    println!("1. Individual QR (Static):");

    let info = IndividualInfo::builder()
        .bakong_account_id("sliden@bkrt")
        .merchant_name("goon")
        .build()
        .unwrap();

    let result = khqr.generate_qr(info).unwrap();
    println!("   QR String: {}", result.qr);
    println!("   MD5 Hash:  {}", result.md5);
    println!();

    // --------------------------------------------------
    // 2. Generate Individual QR (Dynamic - with amount)
    // --------------------------------------------------
    println!("2. Individual QR (Dynamic with amount):");

    let info = IndividualInfo::builder()
        .bakong_account_id("user@bank")
        .merchant_name("Coffee Shop")
        .merchant_city("Phnom Penh")
        .currency("KHR")
        .amount(50000.0)
        .bill_number("INV-001")
        .build()
        .unwrap();

    let result = khqr.generate_qr(info).unwrap();
    println!("   QR String: {}", result.qr);
    println!("   MD5 Hash:  {}", result.md5);
    println!();

    // --------------------------------------------------
    // 3. Generate Individual QR (USD)
    // --------------------------------------------------
    println!("3. Individual QR (USD):");

    let info = IndividualInfo::builder()
        .bakong_account_id("user@bank")
        .merchant_name("Coffee Shop")
        .merchant_city("Phnom Penh")
        .currency("USD")
        .amount(25.50)
        .build()
        .unwrap();

    let result = khqr.generate_qr(info).unwrap();
    println!("   QR String: {}", result.qr);
    println!("   MD5 Hash:  {}", result.md5);
    println!();

    // --------------------------------------------------
    // 4. Generate Merchant QR
    // --------------------------------------------------
    println!("4. Merchant QR:");

    let info = MerchantInfo::builder()
        .bakong_account_id("merchant@aba")
        .merchant_id("MERCHANT001")
        .acquiring_bank("ABA Bank")
        .merchant_name("Big Store Phnom Penh")
        .merchant_city("Phnom Penh")
        .currency("USD")
        .amount(100.00)
        .bill_number("INV-2024-001")
        .build()
        .unwrap();

    let result = khqr.generate_merchant_qr(info).unwrap();
    println!("   QR String: {}", result.qr);
    println!("   MD5 Hash:  {}", result.md5);
    println!();

    // --------------------------------------------------
    // 5. Generate QR with additional info
    // --------------------------------------------------
    println!("5. QR with additional info:");

    let info = IndividualInfo::builder()
        .bakong_account_id("user@bank")
        .merchant_name("Restaurant")
        .merchant_city("Phnom Penh")
        .currency("KHR")
        .amount(250000.0)
        .bill_number("TABLE-5")
        .mobile_number("85512345678")
        .store_label("Main Branch")
        .terminal_label("POS-01")
        .purpose_of_transaction("Dinner payment")
        .build()
        .unwrap();

    let result = khqr.generate_qr(info).unwrap();
    println!("   QR String: {}", result.qr);
    println!("   MD5 Hash:  {}", result.md5);
    println!();

    println!("=== Done ===");
}
