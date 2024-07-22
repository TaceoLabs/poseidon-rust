// cargo run --release --bin hash_2 -- -a <input_a> -b <input_b>
// e.g., cargo run --release --bin hash_2 -- -a 54939530 -b 190384929

use ark_bn254::Fr;
use clap::Parser;
use num_traits::identities::Zero;
use poseidon_rust::{bn254::circom_t3::POSEIDON_CIRCOM_BN_3_PARAMS, poseidon::Poseidon};
use std::str::FromStr;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// First input (in decimal)
    #[arg(short, long)]
    a: String,

    /// Second input (in decimal)
    #[arg(short, long)]
    b: String,
}

fn main() {
    let args = Args::parse();

    let input_a = Fr::from_str(&args.a).expect("Failed to parse the first input");
    let input_b = Fr::from_str(&args.b).expect("Failed to parse the second input");

    let input = vec![Fr::zero(), input_a, input_b];
    let poseidon = Poseidon::new(&POSEIDON_CIRCOM_BN_3_PARAMS);
    let hash = poseidon
        .permutation(input)
        .expect("Failed to hash the inputs")[0];

    println!("input_a: {}", input_a);
    println!("input_b: {}", input_b);
    println!("hash: {}", hash);
}
