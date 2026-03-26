use khqr::{
    format_amount, format_sub_tag_length_value, format_tag_length_value, md5_hash, pad_length,
    parse_tag_length_value,
};

#[test]
fn test_md5_hash() {
    let hash = md5_hash("test");
    assert_eq!(hash.len(), 32);
}

#[test]
fn test_md5_consistency() {
    let hash1 = md5_hash("hello");
    let hash2 = md5_hash("hello");
    assert_eq!(hash1, hash2);
}

#[test]
fn test_format_amount_khr() {
    let amount = format_amount(50000.0, "KHR");
    assert_eq!(amount, "50000");
}

#[test]
fn test_format_amount_usd() {
    let amount = format_amount(25.99, "USD");
    assert_eq!(amount, "25.99");
}

#[test]
fn test_pad_length() {
    assert_eq!(pad_length("abc"), "03");
    assert_eq!(pad_length("abcdefghij"), "10");
}

#[test]
fn test_format_tag_length_value() {
    let result = format_tag_length_value("00", "01");
    assert_eq!(result, "000201");
}

#[test]
fn test_parse_tag_length_value() {
    let (tag, value) = parse_tag_length_value("000201").unwrap();
    assert_eq!(tag, "00");
    assert_eq!(value, "01");
}

#[test]
fn test_format_sub_tag_length_value() {
    let result = format_sub_tag_length_value("30", "00", "test@bank");
    assert_eq!(result, "0009test@bank");
}
