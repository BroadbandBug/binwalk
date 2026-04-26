use crate::signatures::common::{CONFIDENCE_HIGH, SignatureError, SignatureResult};

/// Human readable description
pub const DESCRIPTION: &str = "WRGG firmware header";

/// WRGG magic bytes
pub fn wrgg_magic() -> Vec<Vec<u8>> {
    vec![
        b"WRGG".to_vec(),
    ]
}

/// Validates WRGG signatures
pub fn wrgg_parser(_file_data: &[u8], offset: usize) -> Result<SignatureResult, SignatureError> {
    Ok(SignatureResult {
        offset,
        confidence: CONFIDENCE_HIGH,
        description: DESCRIPTION.to_string(),
        ..Default::default()
    })
}
