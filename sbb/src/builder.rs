use common::{fingerprint, crypto};
use std::fs;
use std::io::{self, BufRead, Write};
use std::path::Path;
use crate::embed;
use crate::Args;

pub fn secure_binary(args: &Args) -> Result<String, Box<dyn std::error::Error>> {
    println!("[*] Starting secure build for: {}", args.input);

    // Generate output path if not specified
    let output_path = format!("{}.secured", args.input);

    // Get fingerprint - either from key file or generate a new one
    let fp = if args.encrypt {
        // Read from key file
        let key_path = args.key.as_ref().ok_or("Key file required when --encrypt is used")?;
        let file = fs::File::open(key_path)?;
        let mut lines = io::BufReader::new(file).lines();
        let fp = lines.next().ok_or("Key file is empty")??;
        if lines.next().is_some() {
            return Err("Key file should contain only one line".into());
        }
        println!("[+] Using fingerprint from key file: {}", fp);
        fp
    } else {
        // Generate a new fingerprint
        println!("[*] Generating new machine fingerprint...");
        let fp = fingerprint::generate_fingerprint();
        
        // Save the generated fingerprint to a file
        let key_path = format!("{}.key", args.input);
        let mut file = fs::File::create(&key_path)?;
        file.write_all(fp.as_bytes())?;
        println!("[+] Generated fingerprint saved to: {}", key_path);
        
        fp
    };

    // Read binary
    let bin_data = fs::read(&args.input)?;
    println!("[+] Read {} bytes from input binary", bin_data.len());

    // Encrypt binary with fingerprint
    println!("[*] Encrypting binary...");
    let encrypted = crypto::encrypt_binary(&fp, &bin_data)
        .ok_or("Encryption failed")?;
    println!("[+] Encrypted size: {} bytes", encrypted.len());
    
    
    println!("[*] Embedding into stub for target platform...");
    let stub_path = get_stub_path(args);
    println!("[*] Using stub from: {}", stub_path);
    
    // Embed the encrypted binary into the selected stub
    let output_bin = embed::embed_into_stub_with_path(&encrypted, &stub_path)?;
    
    // 5. Save final binary
    fs::write(&output_path, output_bin)?;
    println!("✅ Secured binary written to {}", output_path);

    // Set executable permissions on Unix
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&output_path)?.permissions();
        perms.set_mode(0o755);  // rwxr-xr-x
        fs::set_permissions(&output_path, perms)?;
    }

    Ok(output_path)
}

// Helper function for choosing the right stub path based on target platform
pub fn get_stub_path(args: &Args) -> String {
    let mut stub_path = String::from("/home/ahmed/Projects/collage/sbb/target/release/stub");
    
    // Set platform-specific path if needed
    if args.windows {
        stub_path = String::from("/home/ahmed/Projects/collage/sbb/target/x86_64-pc-windows-gnu/release/stub.exe");
    } else if args.linux {
        // Keep the default Linux stub path
    }
    
    stub_path
}