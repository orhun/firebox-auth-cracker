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

pub fn run(args: &Args) -> Result<Option<String>> {
    let mut reader = BufReader::open(&args.wordlist)?;
    let mut buffer = String::new();

    while let Some(line) = reader.read_line(&mut buffer) {
        let shared_secret = match line {
            Ok(v) => v.trim(),
            Err(e) => {
                eprintln!("Skipping... ({e})");
                continue;
            }
        };
        let success = 1;
        let sess_timeout = 86398;
        let idle_timeout = 0;
        // Hash = SHA1(ts + sn + mac + success + sess-timeout + idle_timeout + shared_secret)
        let sig = sha1::sha1_hash(
            format!(
                "{}{}{}{}{}{}{}",
                args.ts, args.sn, args.mac, success, sess_timeout, idle_timeout, shared_secret
            )
            .as_bytes(),
        );
        println!("Checking {sig} ({shared_secret})");
        if sig == args.sig {
            return Ok(Some(shared_secret.to_string()));
        }
    }

    Ok(None)
}
