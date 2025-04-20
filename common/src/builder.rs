use crate::{fingerprint, crypto, embed};
use std::fs;

pub fn secure_binary(input_path: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("[*] Starting secure build for: {}", input_path);

    // 1. Generate fingerprint
    let fp = fingerprint::generate_fingerprint();
    println!("[+] Fingerprint: {}", fp);

    // 2. Read binary
    let bin_data = fs::read(input_path)?;
    println!("[+] Read {} bytes from input binary", bin_data.len());

    // 3. Encrypt binary with fingerprint
    println!("[*] Encrypting binary...");
    let encrypted = crypto::encrypt_binary(&fp, &bin_data)
        .ok_or("Encryption failed")?;
    println!("[+] Encrypted size: {} bytes", encrypted.len());

    // 4. Embed the encrypted binary into the stub
    println!("[*] Embedding into stub...");
    let output_bin = embed::embed_into_stub(&encrypted)?;

    // 5. Save final binary
    fs::write(output_path, output_bin)?;
    println!("✅ Secured binary written to {}", output_path);

    Ok(())
}
