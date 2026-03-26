use bakong_khqr::{calculate_crc16, KHQRDecoder};

#[test]
fn test_decode_raw_basic() {
    let qr = "00020101021229300013test@bank01091234567895204599953031165802KH5906TestUser6010PhnomPenh6304ABCD";
    let result = KHQRDecoder::decode_raw(qr).unwrap();
    assert!(!result.fields.is_empty());
}

#[test]
fn test_verify_valid_crc() {
    let payload =
        "00020101021229300013test@bank01091234567895204599953031165802KH5906TestUser6010PhnomPenh";
    let crc = calculate_crc16(payload);
    let qr = format!("{}63{}", payload, crc);

    let result = KHQRDecoder::verify(&qr).unwrap();
    assert!(result.is_valid);
}

#[test]
fn test_verify_invalid_crc() {
    let qr = "00020101021229300013test@bank01091234567895204599953031165802KH5906TestUser6010PhnomPenh6304FFFF";

    let result = KHQRDecoder::verify(qr).unwrap();
    assert!(!result.is_valid);
}
