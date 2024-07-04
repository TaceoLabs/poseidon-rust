pub mod bn254;
pub mod error;
pub mod parameters;
pub mod poseidon;

use crate::error::Error;
use ark_bn254::Fr;
use ark_ff::{PrimeField, Zero};
use bn254::{circom_t3::POSEIDON_CIRCOM_BN_3_PARAMS, circom_t4::POSEIDON_CIRCOM_BN_4_PARAMS};
use num_bigint::BigUint;
use num_traits::Num;
use poseidon::Poseidon;

pub fn field_from_hex_string<F: PrimeField>(str: &str) -> Result<F, Error> {
    let tmp = match str.strip_prefix("0x") {
        Some(t) => BigUint::from_str_radix(t, 16),
        None => BigUint::from_str_radix(str, 16),
    };

    let tmp = tmp.map_err(|_| Error::ParseString)?;
    Ok(tmp.into())
}

fn commitment(input: Vec<Fr>) -> Result<Fr, Error> {
    let poseidon = Poseidon::new(&POSEIDON_CIRCOM_BN_4_PARAMS);
    let perm = poseidon.permutation(input)?;
    Ok(perm[0])
}

pub fn guessing_game_commit(guess: u16, address: &str, r: &str) -> Result<Fr, Error> {
    let guess = Fr::from(guess);
    let address = field_from_hex_string(address)?;
    let r = field_from_hex_string(r)?;

    commitment(vec![Fr::zero(), guess, address, r])
}

pub fn poseidon_hash_chain(input: Vec<Fr>) -> Result<Fr, Error> {
    let poseidon = Poseidon::new(&POSEIDON_CIRCOM_BN_3_PARAMS);

    let mut state_vec = vec![Fr::zero(); 3];
    for inp in input {
        state_vec[1] = state_vec[0]; // output of the hash chain
        state_vec[0] = Fr::zero(); // Reset capacity part
        state_vec[2] = inp; // first input part
        state_vec = poseidon.permutation(state_vec)?;
    }

    Ok(state_vec[0])
}

#[cfg(test)]
mod commitment_test {
    use super::*;

    #[test]
    fn known_commitment1() {
        let guess = 5;
        let address = "0x70997970c51812dc3a010c7d01b50e0d17dc79c8";
        let r = "0xa";
        let expected = "0x2346b3b208c9e65959af9824ccab4da69ae27d222204fcf0ace7f725e02e512d";

        let result = guessing_game_commit(guess, address, r).unwrap();
        assert_eq!(result, field_from_hex_string(expected).unwrap());
    }

    #[test]
    fn known_commitment2() {
        let guess = 6;
        let address = "0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC";
        let r = "0xa";
        let expected = "0x1cb75e97aa2b617f4d0c6bf6c99606af77cc899ee8c3e765e48af3b4a4f9cf67";

        let result = guessing_game_commit(guess, address, r).unwrap();
        assert_eq!(result, field_from_hex_string(expected).unwrap());
    }
}
