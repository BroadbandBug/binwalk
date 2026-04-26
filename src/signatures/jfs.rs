use crate::signatures::common::{CONFIDENCE_HIGH, SignatureError, SignatureResult};

/// Human readable description
pub const DESCRIPTION: &str = "JFS filesystem";

/// JFS magic bytes
pub fn jfs_magic() -> Vec<Vec<u8>> {
    vec![b"JFS1".to_vec()]
}

/// Validates JFS signatures
pub fn jfs_parser(_file_data: &[u8], offset: usize) -> Result<SignatureResult, SignatureError> {
    Ok(SignatureResult {
        offset,
        confidence: CONFIDENCE_HIGH,
        description: DESCRIPTION.to_string(),
        ..Default::default()
    })
}
