/// Command-line arguments.
pub mod args;

/// SHA1 wrapper.
pub mod sha1;

/// File reader;
pub mod reader;

/// Error implementation.
pub mod error;

use args::Args;
use error::Result;
use reader::BufReader;
use std::io::{self, BufRead};

/// Checks the secret and returns if it matches the signature.
///
/// Hash = SHA1(ts + sn + mac + success + sess-timeout + idle_timeout + shared_secret)
fn check_secret(secret: &str, args: &Args) -> Option<String> {
    let success = 1;
    let sess_timeout = 86398;
    let idle_timeout = 0;
    let sig = sha1::sha1_hash(
        format!(
            "{}{}{}{}{}{}{}",
            args.ts, args.sn, args.mac, success, sess_timeout, idle_timeout, secret
        )
        .as_bytes(),
    );
    println!("Checking {sig} ({secret})");
    (sig == args.sig).then_some(secret.to_string())
}

/// Main brute force routine.
pub fn run(args: &Args) -> Result<Option<String>> {
    if let Some(input_file) = &args.input_file {
        let mut reader = BufReader::open(input_file)?;
        let mut buffer = String::new();
        while let Some(line) = reader.read_line(&mut buffer) {
            let secret = match line {
                Ok(v) => v.trim().to_string(),
                Err(e) => {
                    eprintln!("Skipping... ({e})");
                    continue;
                }
            };
            if let Some(secret) = check_secret(&secret, args) {
                return Ok(Some(secret));
            }
        }
    } else {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            if let Some(secret) = check_secret(&line?, args) {
                return Ok(Some(secret));
            }
        }
    }
    Ok(None)
}
