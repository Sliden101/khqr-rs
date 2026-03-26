//! Bakong API - Check Account Example
//!
//! This example demonstrates how to check if a Bakong account exists.
//!
//! Prerequisites:
//! - Get a Bakong API token from https://api-bakong.nbc.gov.kh/
//!
//! Run with: BAKONG_TOKEN=your_token cargo run --example api_check_account --features rustls-tls

use khqr::{BakongConfig, BakongKHQR};

#[tokio::main]
async fn main() -> Result<(), khqr::BakongError> {
    println!("=== Bakong API - Check Account Example ===\n");

    // Get token from environment variable
    let token =
        std::env::var("BAKONG_TOKEN").expect("Please set BAKONG_TOKEN environment variable");

    println!("Token: {}\n", token);

    // Create SDK with production config
    let khqr = BakongKHQR::with_config(BakongConfig::production(&token));

    println!("1. Check if account exists:");

    let test_accounts = vec![
        "test@aba",
        "notexist@fakebank",
        "user@wing",
    ];

    for account in test_accounts {
        println!("\n   Checking: {}", account);

        let response = khqr.check_bakong_account(account).await?;

        if let Some(data) = response.data {
            println!("   Exists: {}", data.bakong_account_existed);
        } else {
            println!(
                "   Status: code={}, message={:?}",
                response.status.code, response.status.message
            );
        }
    }

    // --------------------------------------------------
    // 2. Check your own account
    // --------------------------------------------------
    println!("\n\n2. Check your own account (using token account):");

    // You can check the account associated with your token
    // Note: This endpoint might require specific permissions

    println!("\n   Note: Account verification requires appropriate API permissions.");
    println!("   Contact Bakong support to enable this feature.");

    println!("\n=== Done ===");
    Ok(())
}
