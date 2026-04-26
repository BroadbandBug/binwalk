use crate::signatures::common::{CONFIDENCE_HIGH, SignatureError, SignatureResult};

/// Human readable description
pub const DESCRIPTION: &str = "Barebox firmware image";

/// Barebox magic bytes
pub fn barebox_magic() -> Vec<Vec<u8>> {
    vec![
        b"\x23\x62\x62\x78".to_vec(), // #bbx
    ]
}

/// Validates Barebox signatures
pub fn barebox_parser(_file_data: &[u8], offset: usize) -> Result<SignatureResult, SignatureError> {
    Ok(SignatureResult {
        offset,
        confidence: CONFIDENCE_HIGH,
        description: DESCRIPTION.to_string(),
        ..Default::default()
    })
}
