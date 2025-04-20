use sha2::{Digest, Sha256};
use sysinfo::{System, Networks};

/// Generates a unique fingerprint for the current machine
pub fn generate_fingerprint() -> String {
    let mut system = System::new_all();
    system.refresh_all();

    let mut raw_data = String::new();

    // CPU brand or ID
    if let Some(cpu) = system.cpus().get(0) {
        raw_data += cpu.brand();
    }

    // Hostname
    if let Some(name) = System::host_name() {
        raw_data += &name.to_string();
    }

    // MAC address (take first network interface with one)
    let networks = Networks::new_with_refreshed_list();
    for (interface_name, network) in networks.iter() {
        let mac = network.mac_address().to_string();
        if  mac != String::new(){
            raw_data += &mac;
            break;
        }
    }

    // TODO: add disk serials (platform-specific)

    // Hash the final fingerprint
    let mut hasher = Sha256::new();
    hasher.update(raw_data);
    let result = hasher.finalize();

    hex::encode(result)
}