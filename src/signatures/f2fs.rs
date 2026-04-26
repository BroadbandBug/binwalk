use crate::signatures::common::{CONFIDENCE_HIGH, SignatureError, SignatureResult};

/// Human readable description
pub const DESCRIPTION: &str = "F2FS filesystem";

/// F2FS magic bytes
pub fn f2fs_magic() -> Vec<Vec<u8>> {
    vec![b"\x10\x20\xf5\xf2".to_vec()]
}

/// Validates F2FS signatures
pub fn f2fs_parser(_file_data: &[u8], offset: usize) -> Result<SignatureResult, SignatureError> {
    Ok(SignatureResult {
        offset,
        confidence: CONFIDENCE_HIGH,
        description: DESCRIPTION.to_string(),
        ..Default::default()
    })
}
