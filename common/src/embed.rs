

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


// pub fn extract_from_stub(exe: &[u8]) -> Option<Vec<u8>> {
//     // Search for the last occurrence of the start marker
//     let start_positions: Vec<_> = exe.windows(MAGIC_HEADER.len())
//         .enumerate()
//         .filter(|(_, w)| *w == MAGIC_HEADER)
//         .map(|(i, _)| i)
//         .collect();
    
//     // If we found any start markers, use the last one (which should be our appended one)
//     if let Some(&start_pos) = start_positions.last() {
//         println!("Using last start marker at position: {}", start_pos);
        
//         // Calculate where payload begins
//         let payload_start = start_pos + MAGIC_HEADER.len();
//         if payload_start >= exe.len() {
//             return None;
//         }
        
//         // Search for the end marker that follows this start marker
//         let remaining_bytes = &exe[payload_start..];
//         if let Some(end_pos_relative) = remaining_bytes.windows(MAGIC_FOOTER.len())
//             .position(|w| w == MAGIC_FOOTER) 
//         {
//             // Calculate absolute position of end marker
//             let end_pos_absolute = payload_start + end_pos_relative;
            
//             // Extract the payload between markers
//             let payload = &exe[payload_start..end_pos_absolute];
//             println!("Extracted payload of size: {}", payload.len());
            
//             return Some(payload.to_vec());
//         }
//     }
    
//     None
// }

// pub fn extract_from_stub(exe: &[u8]) -> Option<Vec<u8>> {
//     let start = exe.windows(MAGIC_HEADER.len())
//         .position(|w| w == MAGIC_HEADER)?;
//     let end = exe.windows(MAGIC_FOOTER.len())
//         .position(|w| w == MAGIC_FOOTER && w.as_ptr() > &exe[start] as *const _)?;
//     let start = start + MAGIC_HEADER.len();
//     let payload = &exe[start..end];
//     Some(payload.to_vec())
// }

// pub fn extract_from_stub(exe: &[u8]) -> Option<Vec<u8>> {
//    // Search for the last occurrence of the start marker
//     let start_positions: Vec<_> = exe.windows(MAGIC_HEADER.len())
//         .enumerate()
//         .filter(|(_, w)| *w == MAGIC_HEADER)
//         .map(|(i, _)| i)
//         .collect();
    
//     // If we found any start markers, use the last one (which should be our appended one)
//     if let Some(&start_pos) = start_positions.last() {
//         println!("Using last start marker at position: {}", start_pos);
        
//         // Calculate where payload begins
//         let payload_start = start_pos + MAGIC_HEADER.len();
//         if payload_start >= exe.len() {
//             return None;
//         }
        
//         // Search for the end marker that follows this start marker
//         let remaining_bytes = &exe[payload_start..];
//         if let Some(end_pos_relative) = remaining_bytes.windows(MAGIC_FOOTER.len())
//             .position(|w| w == MAGIC_FOOTER) 
//         {
//             // Calculate absolute position of end marker
//             let end_pos_absolute = payload_start + end_pos_relative;
            
//             // Extract the payload between markers
//             let payload = &exe[payload_start..end_pos_absolute];
//             println!("Extracted payload of size: {}", payload.len());
            
//             return Some(payload.to_vec());
//         }
//     }
    
//     None
// }

pub fn extract_from_stub(exe: &[u8]) -> Vec<Vec<u8>> {
    let mut payloads = Vec::new();
    let mut search_start = 0;
    
    loop {
        let remaining_data = &exe[search_start..];
        if let Some(start_pos_relative) = remaining_data.windows(MAGIC_HEADER.len())
            .position(|w| w == MAGIC_HEADER) 
        {
            let start_pos_absolute = search_start + start_pos_relative;
            let payload_start = start_pos_absolute + MAGIC_HEADER.len();
            
            if payload_start >= exe.len() {
                break;
            }
            
            let remaining_bytes = &exe[payload_start..];
            if let Some(end_pos_relative) = remaining_bytes.windows(MAGIC_FOOTER.len())
                .position(|w| w == MAGIC_FOOTER) 
            {
                let payload = &exe[payload_start..payload_start + end_pos_relative];
                payloads.push(payload.to_vec());
                search_start = payload_start + end_pos_relative + MAGIC_FOOTER.len();
            } else {
                break;
            }
        } else {
            break;
        }
    }
    
    payloads
}



// Keep existing tests and add new ones for multiple payload functionality
#[cfg(test)]
mod tests {
    use super::*;

    fn create_mock_stub() -> Vec<u8> {
        b"Mock stub binary content".to_vec()
    }

    #[test]
    fn test_extract_multiple_payloads() {
        let mut mock_stub = create_mock_stub();
        
        let payload1 = b"First payload";
        let payload2 = b"Second payload";
        let payload3 = b"Third payload";
        
        // Embed multiple payloads
        mock_stub.extend_from_slice(MAGIC_HEADER);
        mock_stub.extend_from_slice(payload1);
        mock_stub.extend_from_slice(MAGIC_FOOTER);
        
        mock_stub.extend_from_slice(MAGIC_HEADER);
        mock_stub.extend_from_slice(payload2);
        mock_stub.extend_from_slice(MAGIC_FOOTER);
        
        mock_stub.extend_from_slice(MAGIC_HEADER);
        mock_stub.extend_from_slice(payload3);
        mock_stub.extend_from_slice(MAGIC_FOOTER);
        
        let extracted = extract_from_stub(&mock_stub);
        assert_eq!(extracted.len(), 3);
        assert_eq!(extracted[0], payload1);
        assert_eq!(extracted[1], payload2);
        assert_eq!(extracted[2], payload3);
    }

    // Keep existing tests...
}