use crate::signatures::common::{CONFIDENCE_HIGH, SignatureError, SignatureResult};

/// Human readable description
pub const DESCRIPTION: &str = "CSYS firmware header";

/// CSYS magic bytes
pub fn csys_magic() -> Vec<Vec<u8>> {
    vec![
        b"CSYS".to_vec(),
    ]
}

/// Validates CSYS signatures
pub fn csys_parser(_file_data: &[u8], offset: usize) -> Result<SignatureResult, SignatureError> {
    Ok(SignatureResult {
        offset,
        confidence: CONFIDENCE_HIGH,
        description: DESCRIPTION.to_string(),
        ..Default::default()
    })
}
