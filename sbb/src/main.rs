mod builder;
mod fingerprint;
mod crypto;
mod embed;

use clap::Parser;

/// Secure Binary Builder
#[derive(clap::Parser)]
struct Args {
    /// Path to the binary to secure
    input: String,

    /// Output path for secured binary
    output: String,
}

fn main() {
    let args = Args::parse();

    match builder::secure_binary(&args.input, &args.output) {
        Ok(_) => println!("✅ Secured binary written to {}", args.output),
        Err(e) => eprintln!("❌ Error: {}", e),
    }
}
