// cargo run --release --bin commitment -- --guess <GUESS> --rand <RAND> --address <ADDRESS>
// e.g., cargo run --release --bin commitment -- --guess 5 --rand 0xa --address 0x70997970c51812dc3a010c7d01b50e0d17dc79c8

use clap::Parser;
use num_bigint::BigUint;
use poseidon_rust::guessing_game_commit;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// The guess
    #[arg(short, long)]
    guess: u16,

    /// randomness as hexstring
    #[arg(short, long)]
    rand: String,

    /// randomness as hexstring
    #[arg(short, long)]
    address: String,
}

fn main() {
    let args = Args::parse();

    let commitment = guessing_game_commit(args.guess, &args.address, &args.rand);
    let commitment = match commitment {
        Ok(c) => c,
        Err(e) => {
            println!("Failed to parse the inputs: {:?}", e);
            std::process::exit(1);
        }
    };
    let biguint: BigUint = commitment.into(); // For output in hex

    println!("guess: {}", args.guess);
    println!("address: {}", args.address);
    println!("rand: {}", args.rand);
    println!("commitment: 0x{}", biguint.to_str_radix(16));
}
