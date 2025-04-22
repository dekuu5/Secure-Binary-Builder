// src/embed.rs
use std::fs;

const MAGIC_HEADER: &[u8] = b"--EMBED_START--";
const MAGIC_FOOTER: &[u8] = b"--EMBED_END--";

pub fn embed_into_stub(payload: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut stub = std::fs::read("/home/ahmed/Projects/collage/sbb/target/debug/stub")?; // should be a clean copy!
    stub.extend_from_slice(MAGIC_HEADER);
    stub.extend_from_slice(payload);
    stub.extend_from_slice(MAGIC_FOOTER);
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
