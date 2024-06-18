use otpauth::TOTP;
use std::time::{SystemTime, UNIX_EPOCH};
use clap::Parser;

#[derive(Parser)]
#[command(name = "totp")]
#[command(version = "1.0")]
#[command(about = "tool", long_about = None)]
struct Cli {
    #[arg(long)]
    secret: String,
}

fn main() {
    let cli = Cli::parse();
    let code = get_otp(cli.secret);
    println!("{:?}", code);
}

fn get_otp(secret: String) -> u32 {
    let auth = TOTP::from_base32(secret).unwrap();
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    return auth.generate(30, timestamp);
}