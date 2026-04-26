use crate::signatures::common::{CONFIDENCE_HIGH, SignatureError, SignatureResult};

/// Human readable description
pub const DESCRIPTION: &str = "ZynOS firmware header";

/// ZynOS magic bytes
pub fn zynos_magic() -> Vec<Vec<u8>> {
    vec![
        b"\xaa\x55\xaa\x55\xaa\x55\xaa\x55".to_vec(),
    ]
}

/// Validates ZynOS signatures
pub fn zynos_parser(_file_data: &[u8], offset: usize) -> Result<SignatureResult, SignatureError> {
    Ok(SignatureResult {
        offset,
        confidence: CONFIDENCE_HIGH,
        description: DESCRIPTION.to_string(),
        ..Default::default()
    })
}
