use khqr::{calculate_crc16, verify_crc};

#[test]
fn test_crc_basic() {
    let data =
        "00020101021229300013test@bank01091234567895204599953031165802KH5906TestUser6010PhnomPenh";
    let crc = calculate_crc16(data);
    assert_eq!(crc.len(), 4);
}

#[test]
fn test_crc_consistency() {
    let data = "00020101021229300013test@bank";
    let crc1 = calculate_crc16(data);
    let crc2 = calculate_crc16(data);
    assert_eq!(crc1, crc2);
}

#[test]
fn test_verify_crc_valid() {
    let payload =
        "00020101021229300013test@bank01091234567895204599953031165802KH5906TestUser6010PhnomPenh";
    let expected_crc = calculate_crc16(payload);
    let qr = format!("{}63{}", payload, expected_crc);

    let (is_valid, calc_crc, actual) = verify_crc(&qr);
    assert!(
        is_valid,
        "CRC should be valid, expected: {}, actual: {}",
        calc_crc, actual
    );
}

#[test]
fn test_verify_crc_invalid() {
    let qr = "00020101021229300013test@bank01091234567895204599953031165802KH5906TestUser6010PhnomPenh6304FFFF";
    let (is_valid, _, _) = verify_crc(qr);
    assert!(!is_valid, "CRC should be invalid");
}
