use crate::signatures::common::{CONFIDENCE_HIGH, SignatureError, SignatureResult};

/// Human readable description
pub const DESCRIPTION: &str = "WebP image data";

/// WebP magic bytes (RIFF .... WEBP)
pub fn webp_magic() -> Vec<Vec<u8>> {
    vec![b"RIFF".to_vec()]
}

/// Validates WebP signatures
pub fn webp_parser(file_data: &[u8], offset: usize) -> Result<SignatureResult, SignatureError> {
    // RIFF header is at least 12 bytes
    if file_data.len() < offset + 12 {
        return Err(SignatureError);
    }

    // Check for "WEBP" at offset 8
    if &file_data[offset + 8..offset + 12] != b"WEBP" {
        return Err(SignatureError);
    }

    Ok(SignatureResult {
        offset,
        confidence: CONFIDENCE_HIGH,
        description: DESCRIPTION.to_string(),
        ..Default::default()
    })
}
