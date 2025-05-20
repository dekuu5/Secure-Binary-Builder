// src/embed.rs
use std::{fs, path::Path};

const MAGIC_HEADER: &[u8] = b"--EMBED_START--";
const MAGIC_FOOTER: &[u8] = b"--EMBED_END--";

pub fn embed_into_stub(payload: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut stub = std::fs::read("/home/ahmed/Projects/collage/sbb/target/debug/stub")?; // should be a clean copy!
    stub.extend_from_slice(MAGIC_HEADER);
    stub.extend_from_slice(payload);
    stub.extend_from_slice(MAGIC_FOOTER);
    Ok(stub)
}

pub fn embed_into_stub_with_path(payload: &[u8], stub_path: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // Check if stub exists
    if !Path::new(stub_path).exists() {
        return Err(format!("Stub binary not found at path: {}", stub_path).into());
    }
    
    // Read the stub binary
    let mut stub = std::fs::read(stub_path)?;
    println!("Read stub binary from: {} (size: {} bytes)", stub_path, stub.len());
    
    // Append the header, payload, and footer
    stub.extend_from_slice(MAGIC_HEADER);
    println!("Added header at position: {}", stub.len() - MAGIC_HEADER.len());
    
    stub.extend_from_slice(payload);
    println!("Added payload at position: {}, size: {}", 
             stub.len() - payload.len(), payload.len());
    
    stub.extend_from_slice(MAGIC_FOOTER);
    println!("Added footer at position: {}", stub.len() - MAGIC_FOOTER.len());
    
    println!("Final binary size: {} bytes", stub.len());
    
    Ok(stub)
}

pub fn extract_from_stub(exe: &[u8]) -> Option<Vec<u8>> {
    let start = exe.windows(MAGIC_HEADER.len())
        .position(|w| w == MAGIC_HEADER)?;
    let end = exe.windows(MAGIC_FOOTER.len())
        .position(|w| w == MAGIC_FOOTER && w.as_ptr() > &exe[start] as *const _)?;
    let start = start + MAGIC_HEADER.len();
    let payload = &exe[start..end];
    Some(payload.to_vec())
}
