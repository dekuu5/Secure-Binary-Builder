use sha2::{Digest, Sha256};
use sysinfo::{System, Networks};
use rand::Rng;


pub fn generate_fingerprint() -> String {
    let mut system = System::new_all();
    system.refresh_all();

    let mut raw_data = String::new();

    // Add CPU brand
    if let Some(cpu) = system.cpus().get(0) {
        raw_data += cpu.brand();
    }

    // Add hostname
    if let Some(name) = System::host_name() {
        raw_data += &name;
    }

    // Add first valid MAC address that is not all zeros
    let networks = Networks::new_with_refreshed_list();
    for (_name, net) in networks.iter() {
        let mac = net.mac_address();
        let mac_str = mac.to_string();
        if mac_str != "00:00:00:00:00:00" {
            raw_data += &mac_str;
            break;
        }
    }

    // fallback
    if raw_data.is_empty() {
        raw_data = "fallback".to_string(); // prevent hashing empty data
    }

    let hash = Sha256::digest(raw_data.as_bytes());
    hex::encode(hash)
}


pub fn generate_random_key() -> String {
    println!("[*] Generating random encryption key...");
    let mut rng = rand::rng();
    
    let random_bytes: Vec<u8> = (0..128).map(|_| rng.random()).collect();
    let hash = Sha256::digest(&random_bytes);
    let fp = hex::encode(hash);
    println!("random fp is {}",fp);
    fp
}