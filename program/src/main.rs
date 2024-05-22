//! A simple program that takes a number `n` as input, and writes the `n-1`th and `n`th fibonacci
//! number as an output.

// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);

use alloy_sol_types::{sol, SolType};
use reth_primitives::{Signature, B256};
use std::str::FromStr;

/// The public values encoded as a tuple that can be easily deserialized inside Solidity.
type PublicValuesTuple = sol! {
    address
};

pub fn main() {
    let hash =
        B256::from_str("daf5a779ae972f972197303d7b574746c7ef83eadac0f2791ad23db92e4c8e53").unwrap();

    let signature = sp1_zkvm::io::read::<Signature>();

    let signer = signature.recover_signer(hash).unwrap();

    // Encocde the public values of the program.
    let bytes = PublicValuesTuple::abi_encode(&signer);

    // Commit to the public values of the program.
    sp1_zkvm::io::commit_slice(&bytes);

    // Print out the public values.
    println!("value: {}", signer);
}
