use crate::signatures::common::{CONFIDENCE_HIGH, SignatureError, SignatureResult};

/// Human readable description
pub const DESCRIPTION: &str = "HFS filesystem";

/// HFS magic bytes
pub fn hfs_magic() -> Vec<Vec<u8>> {
    vec![
        b"BD".to_vec(), // HFS
        b"H+".to_vec(), // HFS+
        b"HX".to_vec(), // HFSX
    ]
}

/// Validates HFS signatures
pub fn hfs_parser(_file_data: &[u8], offset: usize) -> Result<SignatureResult, SignatureError> {
    // HFS magic is at offset 1024 from the start of the volume
    if offset < 1024 {
        return Err(SignatureError);
    }

    Ok(SignatureResult {
        offset: offset - 1024,
        confidence: CONFIDENCE_HIGH,
        description: DESCRIPTION.to_string(),
        ..Default::default()
    })
}
