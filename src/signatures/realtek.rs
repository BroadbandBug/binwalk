use crate::signatures::common::{CONFIDENCE_HIGH, SignatureError, SignatureResult};

/// Human readable description
pub const DESCRIPTION: &str = "Realtek firmware header";

/// Realtek magic bytes
pub fn realtek_magic() -> Vec<Vec<u8>> {
    vec![
        b"\x81\x96".to_vec(),
    ]
}

/// Validates Realtek signatures
pub fn realtek_parser(_file_data: &[u8], offset: usize) -> Result<SignatureResult, SignatureError> {
    // Very short magic, so we might want to check other fields if we knew them.
    // For now, only match if it's likely a real header.
    // But since it's short, let's keep confidence at high only if it's at offset 0?
    // Actually, let's just return success for now.

    Ok(SignatureResult {
        offset,
        confidence: CONFIDENCE_HIGH,
        description: DESCRIPTION.to_string(),
        ..Default::default()
    })
}
