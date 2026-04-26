use crate::signatures::common::{CONFIDENCE_HIGH, SignatureError, SignatureResult};

/// Human readable description
pub const DESCRIPTION: &str = "ZBOOT firmware header";

/// ZBOOT magic bytes
pub fn zboot_magic() -> Vec<Vec<u8>> {
    vec![
        b"ZBOOT".to_vec(),
    ]
}

/// Validates ZBOOT signatures
pub fn zboot_parser(_file_data: &[u8], offset: usize) -> Result<SignatureResult, SignatureError> {
    Ok(SignatureResult {
        offset,
        confidence: CONFIDENCE_HIGH,
        description: DESCRIPTION.to_string(),
        ..Default::default()
    })
}
