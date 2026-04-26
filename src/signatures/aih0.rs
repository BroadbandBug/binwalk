use crate::signatures::common::{CONFIDENCE_HIGH, SignatureError, SignatureResult};

/// Human readable description
pub const DESCRIPTION: &str = "AIH0 firmware header";

/// AIH0 magic bytes
pub fn aih0_magic() -> Vec<Vec<u8>> {
    vec![
        b"AIH0".to_vec(),
    ]
}

/// Validates AIH0 signatures
pub fn aih0_parser(_file_data: &[u8], offset: usize) -> Result<SignatureResult, SignatureError> {
    Ok(SignatureResult {
        offset,
        confidence: CONFIDENCE_HIGH,
        description: DESCRIPTION.to_string(),
        ..Default::default()
    })
}
