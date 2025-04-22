use aes_gcm::{Aes256Gcm, KeyInit, aead::{Aead, generic_array::GenericArray}};
use rand::Rng;
use sha2::{Digest, Sha256};

/// Encrypts the binary using the fingerprint as the key base.
/// Returns `nonce + ciphertext` as a vector of bytes.
pub fn encrypt_binary(fingerprint: &str, data: &[u8]) -> Option<Vec<u8>> {
    // Hash the fingerprint to get a 256-bit key
    let mut hasher = Sha256::new();
    hasher.update(fingerprint.as_bytes());
    let key_bytes = hasher.finalize();

    let key = GenericArray::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);

    // Generate a random 96-bit nonce
    let mut nonce_bytes = [0u8; 12];
    rand::thread_rng().fill(&mut nonce_bytes);
    let nonce = GenericArray::from_slice(&nonce_bytes);

    // Encrypt the binary data
    match cipher.encrypt(nonce, data) {
        Ok(mut ciphertext) => {
            // Prepend nonce to the encrypted data
            let mut output = nonce_bytes.to_vec();
            output.append(&mut ciphertext);
            Some(output)
        }
        Err(_) => None,
    }
}

/// Decrypts a binary using fingerprint-based key.
/// Takes `nonce + ciphertext`, returns decrypted data or None.
pub fn decrypt_binary(fingerprint: &str, encrypted: &[u8]) -> Option<Vec<u8>> {
    if encrypted.len() < 12 {
        return None;
    }

    let nonce = GenericArray::from_slice(&encrypted[0..12]);
    let ciphertext = &encrypted[12..];

    let mut hasher = Sha256::new();
    hasher.update(fingerprint.as_bytes());
    let key_bytes = hasher.finalize();

    let key = GenericArray::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);

    cipher.decrypt(nonce, ciphertext).ok()
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::fingerprint;
    use std::fs;
    use std::os::unix::fs::PermissionsExt;

    #[test]
    fn test_real_binary_encryption_and_decryption() {
        // Use the actual machine fingerprint
        let fp = fingerprint::generate_fingerprint();
        println!("[*] Using fingerprint: {}", fp);

        // Load real binary called "heloE" from the root directory
        let path = "./test1";
        let original = fs::read(path)
            .expect("Failed to read ./heloE binary for test");

        // Encrypt the binar9.14.2
        let encrypted = encrypt_binary(&fp, &original)
            .expect("Encryption failed on real binary");

        // Decrypt it
        let decrypted = decrypt_binary(&fp, &encrypted)
            .expect("Decryption failed");
        // Write the decrypted binary to disk
        let output_path = "./hello_decrypted";
        fs::write(output_path, &decrypted)
            .expect("Failed to write decrypted binary to disk");

        // Set executable permissions on the decrypted file
        let mut permissions = fs::metadata(output_path)
            .expect("Failed to get metadata for decrypted binary")
            .permissions();
        permissions.set_mode(0o755); // rwxr-xr-x
        fs::set_permissions(output_path, permissions)
            .expect("Failed to set executable permissions on decrypted binary");
        // Ensure the decrypted data matches the original binary exactly
        assert_eq!(original, decrypted, "Decrypted binary does not match original");

        println!("✅ Real binary encrypted and decrypted successfully.");
    }
}
