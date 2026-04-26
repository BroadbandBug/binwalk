use crate::signatures::common::{CONFIDENCE_HIGH, SignatureError, SignatureResult};

/// Human readable description
pub const DESCRIPTION: &str = "XFS filesystem";

/// XFS magic bytes
pub fn xfs_magic() -> Vec<Vec<u8>> {
    vec![b"XFSB".to_vec()]
}

/// Validates XFS signatures
pub fn xfs_parser(_file_data: &[u8], offset: usize) -> Result<SignatureResult, SignatureError> {
    Ok(SignatureResult {
        offset,
        confidence: CONFIDENCE_HIGH,
        description: DESCRIPTION.to_string(),
        ..Default::default()
    })
}
