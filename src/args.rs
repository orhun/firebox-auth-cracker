use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Args {
    /// Checksum value.
    #[arg(short, long = "sig")]
    signature: String,

    /// Path of the wordlist that contains shared secret keys.
    #[arg(short, long)]
    wordlist: PathBuf,

    /// Serial number of the Firebox.
    #[arg(long = "sn")]
    serial: String,

    /// MAC address of the client.
    #[arg(short, long)]
    mac: String,
}
