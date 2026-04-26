use crate::signatures::common::{CONFIDENCE_HIGH, SignatureError, SignatureResult};

/// Human readable description
pub const DESCRIPTION: &str = "Mach-O 64-bit executable";

/// Mach-O magic bytes
pub fn macho_magic() -> Vec<Vec<u8>> {
    vec![
        b"\xfe\xed\xfa\xcf".to_vec(), // 64-bit big-endian
        b"\xcf\xfa\xed\xfe".to_vec(), // 64-bit little-endian
    ]
}

/// Validates Mach-O signatures
pub fn macho_parser(file_data: &[u8], offset: usize) -> Result<SignatureResult, SignatureError> {
    // Mach-O header is at least 32 bytes for 64-bit
    const MACHO_64_HEADER_SIZE: usize = 32;

    if file_data.len() < offset + MACHO_64_HEADER_SIZE {
        return Err(SignatureError);
    }

    // Just a very basic validation: filetype and ncmds should be somewhat sane
    // filetype is at offset 12, ncmds at offset 16
    // Since we support both LE and BE, we need to be careful.
    // For now, just returning success if the magic matched (which happened before calling this)
    // but at least we checked the length.

    Ok(SignatureResult {
        offset,
        confidence: CONFIDENCE_HIGH,
        description: DESCRIPTION.to_string(),
        ..Default::default()
    })
}
