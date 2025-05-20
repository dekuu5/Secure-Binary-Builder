mod builder;

mod embed;

use clap::Parser;

/// Secure Binary Builder
#[derive(clap::Parser)]
struct Args {
    /// Path to the binary to secure
    input: String,

    /// Path to the key file (optional, required if --encrypt is set)
    #[arg(value_name = "KEY", required = false)]
    key: Option<String>,

    /// Encrypt the binary using the provided key
    #[arg(long)]
    encrypt: bool,

    /// Target Windows platform
    #[arg(long, group = "platform")]
    windows: bool,

    /// Target Linux platform
    #[arg(long, group = "platform")]
    linux: bool,
}

fn main() {
    let args = Args::parse();

    // Example usage of parsed arguments
    println!("Input: {}", args.input);
    if let Some(key) = &args.key {
        println!("Key: {}", key);
    }
    println!("Encrypt: {}", args.encrypt);
    println!("Target: {}", if args.windows { "Windows" } else if args.linux { "Linux" } else { "Unknown" });

    match builder::secure_binary(&args) {
        Ok(output_path) => println!("✅ Secured binary written " ),
        Err(e) => eprintln!("❌ Error: {}", e),
    }


}
