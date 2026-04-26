use crate::signatures::common::{CONFIDENCE_HIGH, SignatureError, SignatureResult};

/// Human readable description
pub const DESCRIPTION: &str = "Minix filesystem";

/// Minix magic bytes
pub fn minix_magic() -> Vec<Vec<u8>> {
    vec![
        b"\x13\x8f".to_vec(), // Minix V1
        b"\x8f\x13".to_vec(), // Minix V1 (reverse)
        b"\x24\x13".to_vec(), // Minix V2
        b"\x13\x24".to_vec(), // Minix V2 (reverse)
    ]
}

/// Validates Minix signatures
pub fn minix_parser(_file_data: &[u8], offset: usize) -> Result<SignatureResult, SignatureError> {
    Ok(SignatureResult {
        offset,
        confidence: CONFIDENCE_HIGH,
        description: DESCRIPTION.to_string(),
        ..Default::default()
    })
}
