use clap::Parser;
use std::path::PathBuf;

/// Command-line arguments.
#[derive(Parser)]
#[command(author, version, about)]
pub struct Args {
    /// Authentication signature.
    #[arg(short, long = "sig")]
    pub sig: String,

    /// Input file.
    #[arg(short, long, name = "INPUT")]
    pub input_file: Option<PathBuf>,

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
