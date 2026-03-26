//! CRC16-CCITT-FALSE implementation for KHQR verification

use std::sync::LazyLock;

const CRC16_POLYNOMIAL: u16 = 0x1021;
const CRC16_INITIAL: u16 = 0xFFFF;

fn build_crc_table() -> [u16; 256] {
    let mut table = [0u16; 256];
    for i in 0..256 {
        let mut crc: u16 = (i as u16) << 8;
        for _ in 0..8 {
            if (crc & 0x8000) != 0 {
                crc = (crc << 1) ^ CRC16_POLYNOMIAL;
            } else {
                crc = crc << 1;
            }
        }
        table[i] = crc;
    }
    table
}

static CRC_TABLE: LazyLock<[u16; 256]> = LazyLock::new(build_crc_table);

pub fn calculate_crc16(data: &str) -> String {
    let mut crc: u16 = CRC16_INITIAL;
    for byte in data.bytes() {
        let index = ((crc >> 8) as u8) ^ byte;
        crc = (crc << 8) ^ CRC_TABLE[index as usize];
    }
    format!("{:04X}", crc)
}

pub fn verify_crc(qr_string: &str) -> (bool, String, String) {
    if qr_string.len() < 4 {
        return (false, "".to_string(), "QR string too short".to_string());
    }

    let parts: Vec<&str> = qr_string.split("63").collect();
    if parts.len() != 2 {
        return (false, "".to_string(), "Invalid CRC format".to_string());
    }

    let payload = parts[0];
    let actual_crc = parts[1];

    if actual_crc.len() < 4 {
        return (false, "".to_string(), "CRC value too short".to_string());
    }

    let actual_crc_trimmed = &actual_crc[..4];
    let expected_crc = calculate_crc16(payload);

    let is_valid = expected_crc.to_uppercase() == actual_crc_trimmed.to_uppercase();
    (is_valid, expected_crc, actual_crc_trimmed.to_string())
}
