use crate::extractors::pcap::pcapng_carver;
use crate::signatures::common::{CONFIDENCE_HIGH, SignatureError, SignatureResult};

/// Human readable description
pub const PCAP_DESCRIPTION: &str = "Libpcap capture file";
pub const PCAPNG_DESCRIPTION: &str = "Pcap-NG capture file";

/// Pcap magic bytes
pub fn pcap_magic() -> Vec<Vec<u8>> {
    vec![
        b"\xa1\xb2\xc3\xd4".to_vec(), // Standard
        b"\xd4\xc3\xb2\xa1".to_vec(), // Swapped
        b"\xa1\xb2\x3c\x4d".to_vec(), // Nanosecond
        b"\x4d\x3c\xb2\xa1".to_vec(), // Swapped nanosecond
    ]
}

/// Pcap-NG files always start with these bytes
pub fn pcapng_magic() -> Vec<Vec<u8>> {
    vec![b"\x0A\x0D\x0D\x0A".to_vec()]
}

/// Parses and validates the Libpcap file
pub fn pcap_parser(_file_data: &[u8], offset: usize) -> Result<SignatureResult, SignatureError> {
    Ok(SignatureResult {
        offset,
        description: PCAP_DESCRIPTION.to_string(),
        confidence: CONFIDENCE_HIGH,
        ..Default::default()
    })
}

/// Parses and validates the Pcap-NG file
pub fn pcapng_parser(file_data: &[u8], offset: usize) -> Result<SignatureResult, SignatureError> {
    // Successful return value
    let mut result = SignatureResult {
        offset,
        description: PCAPNG_DESCRIPTION.to_string(),
        confidence: CONFIDENCE_HIGH,
        ..Default::default()
    };

    // Do an extraction dry-run
    let dry_run = pcapng_carver(file_data, offset, None);

    // If dry-run was successful, this is almost certianly a valid pcap-ng file
    if dry_run.success {
        if let Some(pcap_size) = dry_run.size {
            // If this file is just a pcap file, no need to carve it out to yet another file on disk
            if offset == 0 && pcap_size == file_data.len() {
                result.extraction_declined = true;
            }

            // Return parser results
            result.size = pcap_size;
            result.description =
                format!("{}, total size: {} bytes", result.description, result.size);
            return Ok(result);
        }
    }

    Err(SignatureError)
}
