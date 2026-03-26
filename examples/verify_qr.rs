//! QR Code Verification Example
//!
//! This example demonstrates how to verify KHQR codes using CRC checksum.
//!
//! Run with: cargo run --example verify_qr --features rustls-tls

use bakong_khqr::{calculate_crc16, verify_crc, KHQRDecoder};

fn main() {
    println!("=== KHQR Code Verification Example ===\n");

    // --------------------------------------------------
    // 1. Verify a valid QR code
    // --------------------------------------------------
    println!("1. Verify valid QR code:");

    let qr_string = "00020101021229300013test@bank01091234567895204599953031165802KH5906TestUser6010PhnomPenh6304ABCD";

    // Calculate expected CRC for this payload
    let payload =
        "00020101021229300013test@bank01091234567895204599953031165802KH5906TestUser6010PhnomPenh";
    let expected_crc = calculate_crc16(payload);
    let valid_qr = format!("{}63{}", payload, expected_crc);

    println!("   Payload:   {}", payload);
    println!("   CRC:       {}", expected_crc);
    println!("   Full QR:   {}", valid_qr);

    // Verify using verify_crc function
    let (is_valid, calc_crc, actual) = verify_crc(&valid_qr);
    println!("   Is Valid:  {}", is_valid);
    println!();

    println!("2. Verify invalid QR code (wrong CRC):");

    let invalid_qr = "00020101021229300013test@bank01091234567895204599953031165802KH5906TestUser6010PhnomPenh6304FFFF";

    let (is_valid, calc_crc, actual) = verify_crc(invalid_qr);
    println!("   QR String: {}", invalid_qr);
    println!("   Is Valid:  {}", is_valid);
    println!("   Expected:  {}", calc_crc);
    println!("   Actual:   {}", actual);
    println!();

    // --------------------------------------------------
    // 3. Verify using KHQRDecoder
    // --------------------------------------------------
    println!("3. Verify using KHQRDecoder:");

    let result = KHQRDecoder::verify(&valid_qr).unwrap();
    println!("   Is Valid:  {}", result.is_valid);
    println!("   Expected:   {}", result.expected_crc);
    println!("   Actual:    {}", result.actual_crc);
    println!();

    // --------------------------------------------------
    // 4. Handle invalid QR
    // --------------------------------------------------
    println!("4. Handle invalid QR:");

    let result = KHQRDecoder::verify(invalid_qr).unwrap();
    if result.is_valid {
        println!("   QR is valid!");
    } else {
        println!("   QR is INVALID!");
        for error in &result.errors {
            println!("   Error: {}", error);
        }
    }
    println!();

    // --------------------------------------------------
    // 5. Edge cases
    // --------------------------------------------------
    println!("5. Edge cases:");

    // Too short
    let short_qr = "0001";
    let (is_valid, _, error) = verify_crc(short_qr);
    println!(
        "   Short QR ({:?}): valid={}",
        short_qr.chars().next(),
        is_valid
    );

    // Empty
    let empty_qr = "";
    let (is_valid, _, error) = verify_crc(empty_qr);
    println!("   Empty QR: valid={}", is_valid);

    println!();
    println!("=== Done ===");
}
