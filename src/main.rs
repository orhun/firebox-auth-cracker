use clap::Parser;
use watchguard_bruteforce::args::Args;

fn main() {
    let args = Args::parse();
    match watchguard_bruteforce::run(&args) {
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
