// src/embed.rs
use std::fs;

/// Embeds encrypted data into the pre-built stub
pub fn embed_into_stub(encrypted_data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // The stub binary (pre-built) is included at compile time!
    #[cfg(target_os = "linux")]
    const STUB_BYTES: &[u8] = include_bytes!("../hello");    
    #[cfg(target_os = "windows")]
    const STUB_BYTES: &[u8] = include_bytes!("stub.exe");

    let mut output = STUB_BYTES.to_vec();
    
    // Append encrypted data with a marker
    let marker = b"SBB_ENCRYPTED_DATA";
    output.extend_from_slice(marker);
    output.extend_from_slice(encrypted_data);

    Ok(output)
}

/// Extracts encrypted data from the final binary
pub fn extract_from_stub(secured_bin: &[u8]) -> Option<Vec<u8>> {
    let marker = b"SBB_ENCRYPTED_DATA";
    if let Some(pos) = secured_bin.windows(marker.len()).position(|w| w == marker) {
        Some(secured_bin[pos + marker.len()..].to_vec())
    } else {
        None
    }
}