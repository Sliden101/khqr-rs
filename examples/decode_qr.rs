//! QR Code Decoding Example
//!
//! This example demonstrates how to decode KHQR codes back to structured data.
//!
//! Run with: cargo run --example decode_qr --features rustls-tls

use bakong_khqr::{BakongKHQR, IndividualInfo, KHQRDecoder};

fn main() {
    println!("=== KHQR Code Decoding Example ===\n");

    // First, generate a QR to decode
    let khqr = BakongKHQR::new("example_token");

    let info = IndividualInfo::builder()
        .bakong_account_id("john_doe@aba")
        .merchant_name("John's Restaurant")
        .merchant_city("Phnom Penh")
        .currency("KHR")
        .amount(150000.0)
        .bill_number("TABLE-12")
        .mobile_number("85512345678")
        .build()
        .unwrap();

    let generated = khqr.generate_qr(info).unwrap();
    let qr_string = &generated.qr;

    println!("Generated QR: {}\n", qr_string);

    // --------------------------------------------------
    // 1. Decode QR string to structured data
    // --------------------------------------------------
    println!("1. Full QR Decoding:");

    let decoded = KHQRDecoder::decode(qr_string).unwrap();

    println!(
        "   Payload Format:      {}",
        decoded.payload_format_indicator
    );
    println!(
        "   Point of Initiation: {:?}",
        decoded.point_of_initiation_method
    );
    println!("   Account Type:        {}", decoded.merchant_account_type);
    println!("   Account Info:        {:?}", decoded.account_information);
    println!("   Merchant ID:         {:?}", decoded.merchant_id);
    println!("   Acquiring Bank:     {:?}", decoded.acquiring_bank);
    println!("   Merchant Name:       {}", decoded.merchant_name);
    println!("   Merchant City:       {}", decoded.merchant_city);
    println!("   Currency:            {}", decoded.currency);
    println!("   Amount:              {:?}", decoded.amount);
    println!("   Country Code:        {}", decoded.country_code);
    println!();

    // --------------------------------------------------
    // 2. Additional Data
    // --------------------------------------------------
    println!("2. Additional Data:");
    println!(
        "   Bill Number:         {:?}",
        decoded.additional_data.bill_number
    );
    println!(
        "   Mobile Number:       {:?}",
        decoded.additional_data.mobile_number
    );
    println!(
        "   Store Label:         {:?}",
        decoded.additional_data.store_label
    );
    println!(
        "   Terminal Label:      {:?}",
        decoded.additional_data.terminal_label
    );
    println!(
        "   Purpose:             {:?}",
        decoded.additional_data.purpose_of_transaction
    );
    println!(
        "   Merchant Ref #:       {:?}",
        decoded.additional_data.merchant_reference_number
    );
    println!();

    // --------------------------------------------------
    // 3. Timestamp info (if present)
    // --------------------------------------------------
    println!("3. Timestamp Info:");
    if let Some(ts) = decoded.timestamp {
        println!("   Creation:     {:?}", ts.creation_timestamp);
        println!("   Expiration:  {:?}", ts.expiration_timestamp);
    } else {
        println!("   No timestamp info");
    }
    println!();

    // --------------------------------------------------
    // 4. Raw field decoding
    // --------------------------------------------------
    println!("4. Raw Field Decoding:");

    let raw = KHQRDecoder::decode_raw(qr_string).unwrap();
    for field in &raw.fields {
        println!("   Tag {}: Value \"{}\"", field.tag, field.value);
    }
    println!();

    // --------------------------------------------------
    // 5. Decode different QR types
    // --------------------------------------------------
    println!("5. Decode Static QR (no amount):");

    let static_info = IndividualInfo::builder()
        .bakong_account_id("user@bank")
        .merchant_name("Static Shop")
        .build()
        .unwrap();

    let static_qr = khqr.generate_qr(static_info).unwrap();
    let decoded_static = KHQRDecoder::decode(&static_qr.qr).unwrap();

    println!(
        "   Point of Initiation: {:?}",
        decoded_static.point_of_initiation_method
    );
    println!("   Amount:              {:?}", decoded_static.amount);
    println!();

    // --------------------------------------------------
    // 6. Error handling
    // --------------------------------------------------
    println!("6. Error handling:");

    let invalid_qr = "00020101021229invalid6304FFFF";
    match KHQRDecoder::decode(invalid_qr) {
        Ok(_) => println!("   Decoded successfully"),
        Err(e) => println!("   Error: {}", e),
    }

    println!();
    println!("=== Done ===");
}
