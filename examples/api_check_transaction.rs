//! Bakong API - Check Transaction Example
//!
//! This example demonstrates how to check transaction status using MD5 hash.
//!
//! Prerequisites:
//! - Get a Bakong API token from https://developer.bakong.kh/
//! - Make a test payment to get a transaction MD5 hash
//!
//! Run with: BAKONG_TOKEN=your_token cargo run --example api_check_transaction --features rustls-tls

use bakong_khqr::{BakongConfig, BakongKHQR, IndividualInfo};

#[tokio::main]
async fn main() -> Result<(), bakong_khqr::BakongError> {
    println!("=== Bakong API - Check Transaction Example ===\n");

    // Get token from environment variable
    let token =
        std::env::var("BAKONG_TOKEN").expect("Please set BAKONG_TOKEN environment variable");

    println!("Token: {}\n", token);

    // Create SDK with production config
    let khqr = BakongKHQR::with_config(BakongConfig::production(&token));

    // --------------------------------------------------
    // 1. Generate a QR code for testing
    // --------------------------------------------------
    println!("1. Generate QR code:");

    let info = IndividualInfo::builder()
        .bakong_account_id("your_account@bank")
        .merchant_name("Test Shop")
        .merchant_city("Phnom Penh")
        .currency("KHR")
        .amount(1000.0)
        .build()
        .unwrap();

    let result = khqr.generate_qr(info).unwrap();
    println!("   QR: {}", result.qr);
    println!("   MD5: {}", result.md5);
    println!();

    // --------------------------------------------------
    // 2. Check transaction by MD5 hash
    // --------------------------------------------------
    println!("2. Check transaction by MD5:");

    let md5_to_check = &result.md5;
    println!("   MD5: {}", md5_to_check);

    let response = khqr.check_transaction_by_md5(md5_to_check).await?;

    if let Some(data) = response.data {
        println!("   Transaction ID:  {:?}", data.transaction_id);
        println!("   Source Account:  {:?}", data.source_account);
        println!("   Dest Account:   {:?}", data.destination_account);
        println!("   Amount:         {:?}", data.amount);
        println!("   Currency:       {:?}", data.currency);
        println!("   Status:         {:?}", data.status);
        println!("   Transaction Date: {:?}", data.transaction_date);
        println!("   Refunded:       {:?}", data.refunded);
    } else {
        println!(
            "   Status: code={}, message={:?}",
            response.status.code, response.status.message
        );
    }
    println!();

    // --------------------------------------------------
    // 3. Check non-existent transaction
    // --------------------------------------------------
    println!("3. Check non-existent transaction:");

    let fake_md5 = "00000000000000000000000000000000";
    println!("   MD5: {}", fake_md5);

    let response = khqr.check_transaction_by_md5(fake_md5).await?;

    if let Some(data) = response.data {
        println!("   Found!");
        println!("   Transaction ID: {:?}", data.transaction_id);
        println!("   Status:        {:?}", data.status);
    } else {
        println!(
            "   Status: code={}, message={:?}",
            response.status.code, response.status.message
        );
    }
    println!();

    // --------------------------------------------------
    // 4. Check transaction by hash
    // --------------------------------------------------
    println!("4. Check transaction by hash (alternative method):");

    let hash = md5_to_check; // Same as MD5 for this use case
    println!("   Hash: {}", hash);

    let response = khqr.check_transaction_by_hash(hash).await?;

    if let Some(data) = response.data {
        println!("   Status: {:?}", data.status);
    } else {
        println!(
            "   Status: code={}, message={:?}",
            response.status.code, response.status.message
        );
    }

    println!("\n=== Done ===");
    Ok(())
}
