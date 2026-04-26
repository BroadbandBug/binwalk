use crate::signatures::common::{CONFIDENCE_HIGH, SignatureError, SignatureResult};

/// Human readable description
pub const DESCRIPTION: &str = "MikroTik NPK firmware";

/// NPK magic bytes
pub fn npk_magic() -> Vec<Vec<u8>> {
    vec![
        b"\x1e\xf1\xee\x2a".to_vec(),
    ]
}

/// Validates NPK signatures
pub fn npk_parser(_file_data: &[u8], offset: usize) -> Result<SignatureResult, SignatureError> {
    Ok(SignatureResult {
        offset,
        confidence: CONFIDENCE_HIGH,
        description: DESCRIPTION.to_string(),
        ..Default::default()
    })
}
