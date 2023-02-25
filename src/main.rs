use clap::Parser;
use firebox_auth_cracker::args::Args;

fn main() {
    let args = Args::parse();
    match firebox_auth_cracker::run(&args) {
        Ok(result) => match result {
            Some(secret) => {
                println!("Secret found!!! -> {secret}")
            }
            None => {
                println!("Secret not found :<")
            }
        },
        Err(e) => {
            eprintln!("{e:?}")
        }
    }
}
