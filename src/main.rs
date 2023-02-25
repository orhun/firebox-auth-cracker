use clap::Parser;
use watchguard_bruteforce::args::Args;

fn main() {
    let args = Args::parse();
    watchguard_bruteforce::run(&args);
}
