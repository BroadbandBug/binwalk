use crate::signatures::common::{CONFIDENCE_HIGH, SignatureError, SignatureResult};

/// Human readable description
pub const DESCRIPTION: &str = "TIFF image data";

/// TIFF magic bytes
pub fn tiff_magic() -> Vec<Vec<u8>> {
    vec![
        b"\x49\x49\x2a\x00".to_vec(), // Little-endian
        b"\x4d\x4d\x00\x2a".to_vec(), // Big-endian
    ]
}

/// Validates TIFF signatures
pub fn tiff_parser(file_data: &[u8], offset: usize) -> Result<SignatureResult, SignatureError> {
    // TIFF header is 8 bytes
    const TIFF_HEADER_SIZE: usize = 8;

    if file_data.len() < offset + TIFF_HEADER_SIZE {
        return Err(SignatureError);
    }

    Ok(SignatureResult {
        offset,
        confidence: CONFIDENCE_HIGH,
        description: DESCRIPTION.to_string(),
        ..Default::default()
    })
}
