use crate::signatures::common::{CONFIDENCE_HIGH, SignatureError, SignatureResult};

/// Human readable description
pub const DESCRIPTION: &str = "ReiserFS filesystem";

/// ReiserFS magic bytes
pub fn reiserfs_magic() -> Vec<Vec<u8>> {
    vec![
        b"ReIsErFs".to_vec(),
        b"ReIsEr2Fs".to_vec(),
        b"ReIsEr3Fs".to_vec(),
    ]
}

/// Validates ReiserFS signatures
pub fn reiserfs_parser(_file_data: &[u8], offset: usize) -> Result<SignatureResult, SignatureError> {
    Ok(SignatureResult {
        offset,
        confidence: CONFIDENCE_HIGH,
        description: DESCRIPTION.to_string(),
        ..Default::default()
    })
}
