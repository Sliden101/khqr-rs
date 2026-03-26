# Bakong KHQR SDK

[![Crates.io](https://img.shields.io/crates/v/khqr)](https://crates.io/crates/khqr)
[![Docs](https://docs.rs/khqr/badge.svg)](https://docs.rs/khqr)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

Unofficial Rust SDK for Bakong KHQR

## Features

- QR Code Generation (Individual & Merchant accounts)
- QR Code Decoding
- CRC16 Verification
- Bakong API Integration
- Support for KHR and USD currencies

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
bakong-khqr = "0.1.0"
```

## Quick Start

```rust
use bakong_khqr::{BakongKHQR, IndividualInfo};

let khqr = BakongKHQR::new("your_token");

let info = IndividualInfo::builder()
    .bakong_account_id("user@bank")
    .merchant_name("Coffee Shop")
    .merchant_city("Phnom Penh")
    .amount(10000.0)
    .build()
    .unwrap();

let result = khqr.generate_qr(info).unwrap();
println!("QR: {}", result.qr);
println!("MD5: {}", result.md5);
```

## Examples

Run the examples:

```bash
# QR Generation
cargo run --example generate_qr --features rustls-tls

# QR Verification
cargo run --example verify_qr --features rustls-tls

# QR Decoding
cargo run --example decode_qr --features rustls-tls

# API - Check Account (requires token)
BAKONG_TOKEN=your_token cargo run --example api_check_account --features rustls-tls

# API - Check Transaction (requires token)
BAKONG_TOKEN=your_token cargo run --example api_check_transaction --features rustls-tls
```

## API Reference

### QR Generation

```rust
// Individual QR
let info = IndividualInfo::builder()
    .bakong_account_id("user@bank")
    .merchant_name("Shop Name")
    .merchant_city("Phnom Penh")
    .currency("KHR")  // or "USD"
    .amount(10000.0)  // optional, omit for static QR
    .build()
    .unwrap();

let result = khqr.generate_qr(info).unwrap();

// Merchant QR
let info = MerchantInfo::builder()
    .bakong_account_id("merchant@bank")
    .merchant_id("MERCHANT001")
    .acquiring_bank("ABA Bank")
    .merchant_name("Store Name")
    .amount(50.00)
    .build()
    .unwrap();

let result = khqr.generate_merchant_qr(info).unwrap();
```

### QR Verification

```rust
use bakong_khqr::{KHQRDecoder, verify_crc};

// Quick verification
let (is_valid, expected, actual) = verify_crc(qr_string);

// Detailed verification
let result = KHQRDecoder::verify(qr_string).unwrap();
if result.is_valid {
    println!("Valid QR code");
}
```

### QR Decoding

```rust
use bakong_khqr::KHQRDecoder;

let decoded = KHQRDecoder::decode(qr_string).unwrap();
println!("Merchant: {}", decoded.merchant_name);
println!("Amount: {:?}", decoded.amount);
println!("Currency: {}", decoded.currency);
```

### API Methods

```rust
// Check if Bakong account exists
let response = khqr.check_bakong_account("user@bank").await?;

// Check transaction by MD5
let response = khqr.check_transaction_by_md5(md5_hash).await?;

// Generate payment deeplink
let response = khqr.generate_deeplink(
    qr_string,
    SourceInfo {
        app_name: "My App".to_string(),
        app_icon_url: None,
        app_deep_link_callback: None,
    },
).await?;
```

## Configuration

### Sandbox (Default)

```rust
let khqr = BakongKHQR::new("sandbox_token");
```

### Production

```rust
use bakong_khqr::BakongConfig;

let khqr = BakongKHQR::with_config(
    BakongConfig::production("production_token")
);
```

### Custom Base URL

```rust
let khqr = BakongKHQR::with_config(
    BakongConfig::sandbox("token")
        .with_base_url("http://localhost:8080")
);
```

## Environment

| Environment | URL |
|-------------|-----|
| Sandbox | https://sit-api-bakong.nbc.gov.kh |
| Production | https://api-bakong.nbc.gov.kh |

## Requirements

- A Bakong API token
- Register at https://api-bakong.nbc.gov.kh/ for sandbox access
- Contact Bakong for production access

## Documentation

- [Official Bakong API Docs](https://api-bakong.nbc.gov.kh/document)
- [KHQR Specification](https://bakong.nbc.gov.kh/)

## License

MIT License - see [LICENSE](LICENSE) for details.


## Contributing

Contributions welcome! Please open an issue or PR on [GitHub](https://github.com/Sliden101/payway-rs)
