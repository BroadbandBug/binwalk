use crate::signatures::common::{CONFIDENCE_HIGH, SignatureError, SignatureResult};

/// Human readable description
pub const DESCRIPTION: &str = "JieLi MCU firmware";

/// JieLi magic bytes
pub fn jieli_magic() -> Vec<Vec<u8>> {
    vec![
        b"\x06\x5a\x00\x00".to_vec(),
    ]
}

/// Validates JieLi signatures
pub fn jieli_parser(_file_data: &[u8], offset: usize) -> Result<SignatureResult, SignatureError> {
    Ok(SignatureResult {
        offset,
        confidence: CONFIDENCE_HIGH,
        description: DESCRIPTION.to_string(),
        ..Default::default()
    })
}
