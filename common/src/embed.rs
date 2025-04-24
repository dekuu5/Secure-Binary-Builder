// src/embed.rs
use std::fs;

const MAGIC_HEADER: &[u8] = b"--EMBED_START--";
const MAGIC_FOOTER: &[u8] = b"--EMBED_END--";

// pub fn embed_into_stub(payload: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
//     let mut stub = std::fs::read("/home/ahmed/Projects/collage/sbb/target/debug/stub")?; // should be a clean copy!
//     stub.extend_from_slice(MAGIC_HEADER);
//     stub.extend_from_slice(payload);
//     stub.extend_from_slice(MAGIC_FOOTER);
//     Ok(stub)
// }

pub fn embed_into_stub(payload: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let stub = std::fs::read("/home/ahmed/Projects/collage/sbb/target/debug/stub")?; // should be a clean copy!
    
    println!("Original stub size: {}", stub.len());
    println!("Payload size: {}", payload.len());
    
    // Create a new buffer to hold the combined data
    let total_size = stub.len() + MAGIC_HEADER.len() + payload.len() + MAGIC_FOOTER.len();
    let mut result = Vec::with_capacity(total_size);
    
    // Copy the stub binary
    result.extend_from_slice(&stub);
    
    // Add the header marker
    result.extend_from_slice(MAGIC_HEADER);
    println!("Added header at position: {}", result.len() - MAGIC_HEADER.len());
    
    // Add the payload
    result.extend_from_slice(payload);
    println!("Added payload at position: {}", result.len() - payload.len());
    
    // Add the footer marker
    result.extend_from_slice(MAGIC_FOOTER);
    println!("Added footer at position: {}", result.len() - MAGIC_FOOTER.len());
    
    println!("Final size: {}", result.len());
    
    Ok(result)
}


pub fn extract_from_stub(exe: &[u8]) -> Option<Vec<u8>> {
    // Search for the last occurrence of the start marker
    let mut start_positions: Vec<_> = exe.windows(MAGIC_HEADER.len())
        .enumerate()
        .filter(|(_, w)| *w == MAGIC_HEADER)
        .map(|(i, _)| i)
        .collect();
    
    // If we found any start markers, use the last one (which should be our appended one)
    if let Some(&start_pos) = start_positions.last() {
        println!("Using last start marker at position: {}", start_pos);
        
        // Calculate where payload begins
        let payload_start = start_pos + MAGIC_HEADER.len();
        if payload_start >= exe.len() {
            return None;
        }
        
        // Search for the end marker that follows this start marker
        let remaining_bytes = &exe[payload_start..];
        if let Some(end_pos_relative) = remaining_bytes.windows(MAGIC_FOOTER.len())
            .position(|w| w == MAGIC_FOOTER) 
        {
            // Calculate absolute position of end marker
            let end_pos_absolute = payload_start + end_pos_relative;
            
            // Extract the payload between markers
            let payload = &exe[payload_start..end_pos_absolute];
            println!("Extracted payload of size: {}", payload.len());
            
            return Some(payload.to_vec());
        }
    }
    
    None
}

// pub fn extract_from_stub(exe: &[u8]) -> Option<Vec<u8>> {
//     let start = exe.windows(MAGIC_HEADER.len())
//         .position(|w| w == MAGIC_HEADER)?;
//     let end = exe.windows(MAGIC_FOOTER.len())
//         .position(|w| w == MAGIC_FOOTER && w.as_ptr() > &exe[start] as *const _)?;
//     let start = start + MAGIC_HEADER.len();
//     let payload = &exe[start..end];
//     Some(payload.to_vec())
// }

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    /// Creates a mock stub binary for testing
    fn create_mock_stub() -> Vec<u8> {
        b"Mock stub binary content".to_vec()
    }

    #[test]
    fn test_embed_extract_roundtrip() {
        // Test data
        let test_payload = b"This is a test payload for embedding";
        
        // Skip if on CI where stub doesn't exist
        if !Path::new("/home/ahmed/Projects/collage/sbb/target/debug/stub").exists() {
            // Use mock stub instead
            let mut mock_stub = create_mock_stub();
            mock_stub.extend_from_slice(MAGIC_HEADER);
            mock_stub.extend_from_slice(test_payload);
            mock_stub.extend_from_slice(MAGIC_FOOTER);
            
            // Extract from mock stub
            let extracted = extract_from_stub(&mock_stub).expect("Failed to extract payload");
            assert_eq!(extracted, test_payload);
        } else {
            
            // Real test with actual stub
            let embedded = embed_into_stub(test_payload).expect("Failed to embed into stub");
            let extracted = extract_from_stub(&embedded).expect("Failed to extract payload");
            assert_eq!(extracted, test_payload);
        }
    }
    
    #[test]
    fn test_extract_nonexistent_marker() {
        let invalid_binary = b"This binary has no markers".to_vec();
        let result = extract_from_stub(&invalid_binary);
        assert!(result.is_none(), "Should return None for binary without markers");
    }
    
    #[test]
    fn test_extract_incomplete_markers() {
        // Only start marker, no end marker
        let mut incomplete = create_mock_stub();
        incomplete.extend_from_slice(MAGIC_HEADER);
        incomplete.extend_from_slice(b"Payload with no end marker");
        let result = extract_from_stub(&incomplete);
        assert!(result.is_none(), "Should return None when end marker is missing");
        
        // Only end marker, no start marker
        let mut incomplete = create_mock_stub();
        incomplete.extend_from_slice(b"Payload with no start marker");
        incomplete.extend_from_slice(MAGIC_FOOTER);
        let result = extract_from_stub(&incomplete);
        assert!(result.is_none(), "Should return None when start marker is missing");
    }
    
    #[test]
    fn test_embed_large_payload() {
        // Create a large test payload (1MB)
        let large_payload = vec![0x41; 1024 * 1024];
        
        if !Path::new("/home/ahmed/Projects/collage/sbb/target/debug/stub").exists() {
            let mut mock_stub = create_mock_stub();
            mock_stub.extend_from_slice(MAGIC_HEADER);
            mock_stub.extend_from_slice(&large_payload);
            mock_stub.extend_from_slice(MAGIC_FOOTER);
            
            let extracted = extract_from_stub(&mock_stub).expect("Failed to extract large payload");
            assert_eq!(extracted.len(), large_payload.len());
            assert_eq!(extracted, large_payload);
        } else {
            let embedded = embed_into_stub(&large_payload).expect("Failed to embed large payload");
            let extracted = extract_from_stub(&embedded).expect("Failed to extract large payload");
            assert_eq!(extracted.len(), large_payload.len());
            assert_eq!(extracted, large_payload);
        }
    }
    
    #[test]
    fn test_extract_with_multiple_markers() {
        // Create a binary with multiple start/end markers, should extract only first complete pair
        let mut multi_marker = create_mock_stub();
        
        // First embedded payload
        let payload1 = b"First payload";
        multi_marker.extend_from_slice(MAGIC_HEADER);
        multi_marker.extend_from_slice(payload1);
        multi_marker.extend_from_slice(MAGIC_FOOTER);
        
        // Second embedded payload (should be ignored)
        multi_marker.extend_from_slice(MAGIC_HEADER);
        multi_marker.extend_from_slice(b"Second payload");
        multi_marker.extend_from_slice(MAGIC_FOOTER);
        
        let extracted = extract_from_stub(&multi_marker).expect("Failed to extract payload");
        assert_eq!(extracted, payload1);
    }
    #[test]
fn test_stub_size_after_embedding() {
    // Test data
    let test_payload = b"This is a test payload for embedding";
    
    if !Path::new("/home/ahmed/Projects/collage/sbb/target/debug/stub").exists() {
        // Mock test if stub doesn't exist
        let mock_stub = create_mock_stub();
        let original_size = mock_stub.len();
        
        let mut embedded = mock_stub.clone();
        embedded.extend_from_slice(MAGIC_HEADER);
        embedded.extend_from_slice(test_payload);
        embedded.extend_from_slice(MAGIC_FOOTER);
        
        // Expected size calculation
        let expected_size = original_size + MAGIC_HEADER.len() + test_payload.len() + MAGIC_FOOTER.len();
        assert_eq!(embedded.len(), expected_size, "Embedded stub size doesn't match expected size");
        
        // Check if we can extract correctly
        let extracted = extract_from_stub(&embedded).expect("Failed to extract payload");
        assert_eq!(extracted, test_payload);
    } else {
        // Real test with actual stub
        let stub_path = "/home/ahmed/Projects/collage/sbb/target/debug/stub";
        let original_size = std::fs::metadata(stub_path).unwrap().len() as usize;
        
        let embedded = embed_into_stub(test_payload).expect("Failed to embed into stub");
        
        // Expected size calculation
        let expected_size = original_size + MAGIC_HEADER.len() + test_payload.len() + MAGIC_FOOTER.len();
        assert_eq!(embedded.len(), expected_size, "Embedded stub size doesn't match expected size");
        
        // Manually output the data to debug
        println!("Original stub size: {}", original_size);
        println!("Embedded size: {}", embedded.len());
        println!("Magic header size: {}", MAGIC_HEADER.len());
        println!("Magic footer size: {}", MAGIC_FOOTER.len());
        println!("Payload size: {}", test_payload.len());
        
        // Print position of markers
        if let Some(start_pos) = embedded.windows(MAGIC_HEADER.len()).position(|w| w == MAGIC_HEADER) {
            println!("Start marker found at position: {}", start_pos);
            let payload_start = start_pos + MAGIC_HEADER.len();
            
            let remaining_bytes = &embedded[payload_start..];
            if let Some(end_pos) = remaining_bytes.windows(MAGIC_FOOTER.len()).position(|w| w == MAGIC_FOOTER) {
                println!("End marker found at relative position: {}", end_pos);
                println!("End marker found at absolute position: {}", payload_start + end_pos);
            } else {
                println!("End marker not found!");
            }
        } else {
            println!("Start marker not found!");
        }
        
        // Check if we can extract correctly
        let extracted = extract_from_stub(&embedded).expect("Failed to extract payload");
        assert_eq!(extracted, test_payload);
    }
}
}