use crate::signatures::common::{CONFIDENCE_HIGH, SignatureError, SignatureResult};

/// Human readable description
pub const DESCRIPTION: &str = "Broadcom firmware header";

/// Broadcom magic bytes
pub fn broadcom_magic() -> Vec<Vec<u8>> {
    vec![
        b"BRCM".to_vec(),
    ]
}

/// Validates Broadcom signatures
pub fn broadcom_parser(_file_data: &[u8], offset: usize) -> Result<SignatureResult, SignatureError> {
    Ok(SignatureResult {
        offset,
        confidence: CONFIDENCE_HIGH,
        description: DESCRIPTION.to_string(),
        ..Default::default()
    })
}
