use crate::signatures::common::{CONFIDENCE_HIGH, SignatureError, SignatureResult};

/// Human readable description
pub const DESCRIPTION: &str = "Dalvik dex file";

/// DEX magic bytes
pub fn dex_magic() -> Vec<Vec<u8>> {
    vec![
        b"dex\n035\x00".to_vec(),
        b"dex\n036\x00".to_vec(),
        b"dex\n037\x00".to_vec(),
        b"dex\n038\x00".to_vec(),
        b"dex\n039\x00".to_vec(),
    ]
}

/// Validates DEX signatures
pub fn dex_parser(_file_data: &[u8], offset: usize) -> Result<SignatureResult, SignatureError> {
    Ok(SignatureResult {
        offset,
        confidence: CONFIDENCE_HIGH,
        description: DESCRIPTION.to_string(),
        ..Default::default()
    })
}
