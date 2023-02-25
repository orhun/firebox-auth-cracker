use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Args {
    /// Checksum value.
    #[arg(short, long = "sig")]
    pub sig: String,

    /// Path of the wordlist that contains shared secret keys.
    #[arg(short, long)]
    pub wordlist: Option<PathBuf>,

    /// Serial number of the Firebox.
    #[arg(long = "sn")]
    pub sn: String,

    /// Timestamp for the request.
    #[arg(short, long = "ts")]
    pub ts: String,

    /// MAC address of the client.
    #[arg(short, long)]
    pub mac: String,
}
