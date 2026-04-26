use crate::signatures::common::{CONFIDENCE_HIGH, SignatureError, SignatureResult};

/// Human readable description
pub const DESCRIPTION: &str = "Java class file";

/// Java class magic bytes
pub fn java_magic() -> Vec<Vec<u8>> {
    vec![b"\xca\xfe\xba\xbe".to_vec()]
}

/// Validates Java class signatures
pub fn java_parser(_file_data: &[u8], offset: usize) -> Result<SignatureResult, SignatureError> {
    Ok(SignatureResult {
        offset,
        confidence: CONFIDENCE_HIGH,
        description: DESCRIPTION.to_string(),
        ..Default::default()
    })
}
