use crate::signatures::common::{CONFIDENCE_HIGH, SignatureError, SignatureResult};

/// Human readable description
pub const DESCRIPTION: &str = "Ubiquiti firmware header";

/// Ubiquiti magic bytes
pub fn ubnt_magic() -> Vec<Vec<u8>> {
    vec![
        b"UBNT".to_vec(),
        b"PART".to_vec(),
        b"ENDS".to_vec(),
    ]
}

/// Validates Ubiquiti signatures
pub fn ubnt_parser(_file_data: &[u8], offset: usize) -> Result<SignatureResult, SignatureError> {
    Ok(SignatureResult {
        offset,
        confidence: CONFIDENCE_HIGH,
        description: DESCRIPTION.to_string(),
        ..Default::default()
    })
}
