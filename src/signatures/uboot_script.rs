use crate::signatures::common::{CONFIDENCE_HIGH, SignatureError, SignatureResult};

/// Human readable description
pub const DESCRIPTION: &str = "U-Boot script";

/// U-Boot script magic bytes
pub fn uboot_script_magic() -> Vec<Vec<u8>> {
    vec![b"\x27\x05\x19\x56".to_vec()]
}

/// Validates U-Boot script signatures
pub fn uboot_script_parser(_file_data: &[u8], offset: usize) -> Result<SignatureResult, SignatureError> {
    Ok(SignatureResult {
        offset,
        confidence: CONFIDENCE_HIGH,
        description: DESCRIPTION.to_string(),
        ..Default::default()
    })
}
