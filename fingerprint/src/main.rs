use std::fs::File;
use std::io::Write;
use std::path::Path;
use common::fingerprint::generate_fingerprint;
fn main() {
    // Assuming the function is defined as: common::generate_fingerprint() -> String
    let fingerprint = generate_fingerprint();

    let path = Path::new("key.txt");
    let mut file = File::create(&path).expect("Unable to create file");
    file.write_all(fingerprint.as_bytes()).expect("Unable to write data");
}