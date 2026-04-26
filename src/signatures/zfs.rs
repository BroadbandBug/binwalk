use crate::signatures::common::{CONFIDENCE_HIGH, SignatureError, SignatureResult};

/// Human readable description
pub const DESCRIPTION: &str = "ZFS filesystem";

/// ZFS magic bytes
pub fn zfs_magic() -> Vec<Vec<u8>> {
    vec![
        b"\x00\x02\x4b\x5a\x00\x00\x00\x00".to_vec(), // LE
        b"\x5a\x4b\x02\x00\x00\x00\x00\x00".to_vec(), // BE
    ]
}

/// Validates ZFS signatures
pub fn zfs_parser(_file_data: &[u8], offset: usize) -> Result<SignatureResult, SignatureError> {
    Ok(SignatureResult {
        offset,
        confidence: CONFIDENCE_HIGH,
        description: DESCRIPTION.to_string(),
        ..Default::default()
    })
}
