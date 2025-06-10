// src/embed.rs
use std::{ path::Path};
use std::fs;
use std::io::Write;
use tempfile::NamedTempFile;

const MAGIC_HEADER: &[u8] = b"--EMBED_START--";
const MAGIC_FOOTER: &[u8] = b"--EMBED_END--";

pub fn embed_multiple_into_stub(payloads: &[Vec<u8>], stub_path: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // Check if stub exists
    if !Path::new(stub_path).exists() {
        return Err(format!("Stub binary not found at path: {}", stub_path).into());
    }
    
    // Read the stub binary
    let mut result = std::fs::read(stub_path)?;
    println!("Read stub binary from: {} (size: {} bytes)", stub_path, result.len());
    
    // Embed each payload with its own markers
    for (index, payload) in payloads.iter().enumerate() {
        println!("Embedding payload {} (size: {} bytes)", index, payload.len());
        
        // Add the header marker
        result.extend_from_slice(MAGIC_HEADER);
        println!("Added header for payload {} at position: {}", index, result.len() - MAGIC_HEADER.len());
        
        // Add the payload
        result.extend_from_slice(payload);
        println!("Added payload {} at position: {}", index, result.len() - payload.len());
        
        // Add the footer marker
        result.extend_from_slice(MAGIC_FOOTER);
        println!("Added footer for payload {} at position: {}", index, result.len() - MAGIC_FOOTER.len());
    }
    
    println!("Final binary size: {} bytes", result.len());
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_embed_single_payload() {
        // Create a temporary stub file
        let mut stub_file = NamedTempFile::new().unwrap();
        let stub_data = b"STUB_BINARY_DATA";
        stub_file.write_all(stub_data).unwrap();
        
        let payload = vec![0x41, 0x42, 0x43]; // "ABC"
        let payloads = vec![payload.clone()];
        
        let result = embed_multiple_into_stub(&payloads, stub_file.path().to_str().unwrap()).unwrap();
        
        // Verify the result contains stub + header + payload + footer
        let expected_size = stub_data.len() + MAGIC_HEADER.len() + payload.len() + MAGIC_FOOTER.len();
        assert_eq!(result.len(), expected_size);
        
        // Verify stub data is at the beginning
        assert_eq!(&result[..stub_data.len()], stub_data);
        
        // Verify header is present
        let header_start = stub_data.len();
        assert_eq!(&result[header_start..header_start + MAGIC_HEADER.len()], MAGIC_HEADER);
        
        // Verify payload is present
        let payload_start = header_start + MAGIC_HEADER.len();
        assert_eq!(&result[payload_start..payload_start + payload.len()], &payload);
        
        // Verify footer is present
        let footer_start = payload_start + payload.len();
        assert_eq!(&result[footer_start..footer_start + MAGIC_FOOTER.len()], MAGIC_FOOTER);
    }

    #[test]
    fn test_embed_multiple_payloads() {
        let mut stub_file = NamedTempFile::new().unwrap();
        let stub_data = b"STUB";
        stub_file.write_all(stub_data).unwrap();
        
        let payload1 = vec![0x01, 0x02];
        let payload2 = vec![0x03, 0x04, 0x05];
        let payloads = vec![payload1.clone(), payload2.clone()];
        
        let result = embed_multiple_into_stub(&payloads, stub_file.path().to_str().unwrap()).unwrap();
        
        // Calculate expected size
        let expected_size = stub_data.len() + 
            2 * (MAGIC_HEADER.len() + MAGIC_FOOTER.len()) + 
            payload1.len() + payload2.len();
        assert_eq!(result.len(), expected_size);
        
        // Verify both payloads are embedded with their markers
        let mut pos = stub_data.len();
        
        // First payload
        assert_eq!(&result[pos..pos + MAGIC_HEADER.len()], MAGIC_HEADER);
        pos += MAGIC_HEADER.len();
        assert_eq!(&result[pos..pos + payload1.len()], &payload1);
        pos += payload1.len();
        assert_eq!(&result[pos..pos + MAGIC_FOOTER.len()], MAGIC_FOOTER);
        pos += MAGIC_FOOTER.len();
        
        // Second payload
        assert_eq!(&result[pos..pos + MAGIC_HEADER.len()], MAGIC_HEADER);
        pos += MAGIC_HEADER.len();
        assert_eq!(&result[pos..pos + payload2.len()], &payload2);
        pos += payload2.len();
        assert_eq!(&result[pos..pos + MAGIC_FOOTER.len()], MAGIC_FOOTER);
    }

    #[test]
    fn test_embed_empty_payloads() {
        let mut stub_file = NamedTempFile::new().unwrap();
        let stub_data = b"STUB";
        stub_file.write_all(stub_data).unwrap();
        
        let payloads: Vec<Vec<u8>> = vec![];
        
        let result = embed_multiple_into_stub(&payloads, stub_file.path().to_str().unwrap()).unwrap();
        
        // Should just return the stub data unchanged
        assert_eq!(result.len(), stub_data.len());
        assert_eq!(result, stub_data);
    }

    #[test]
    fn test_embed_nonexistent_stub() {
        let payloads = vec![vec![0x01, 0x02]];
        let result = embed_multiple_into_stub(&payloads, "/nonexistent/path");
        
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Stub binary not found"));
    }

    #[test]
    fn test_embed_large_payload() {
        let mut stub_file = NamedTempFile::new().unwrap();
        stub_file.write_all(b"STUB").unwrap();
        
        // Create a large payload (1MB)
        let large_payload = vec![0xAA; 1024 * 1024];
        let payloads = vec![large_payload.clone()];
        
        let result = embed_multiple_into_stub(&payloads, stub_file.path().to_str().unwrap()).unwrap();
        
        let expected_size = 4 + MAGIC_HEADER.len() + large_payload.len() + MAGIC_FOOTER.len();
        assert_eq!(result.len(), expected_size);
        
        // Verify the large payload is correctly embedded
        let payload_start = 4 + MAGIC_HEADER.len();
        assert_eq!(&result[payload_start..payload_start + large_payload.len()], &large_payload);
    }
}
