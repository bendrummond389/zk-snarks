mod circuits;
mod r1cs;
mod utils;
mod zk_proofs;

use circuits::Circuit;
use r1cs::r1cs::R1CS;
use std::{collections::HashMap, env};
use utils::polynomial::polynomial::Polynomial;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let file_path = "./circuits/sample_circuits/circuit1.json";

    let mut inputs = HashMap::new();
    inputs.insert("1".to_string(), 1);
    inputs.insert("x".to_string(), 3);

    let mut circuit = Circuit::from_file(file_path).expect("Failed to load circuit");
    let variable_map = circuit.hash_and_index_circuit();

    let mut r1cs = R1CS::new(variable_map);
    r1cs.generate_r1cs_constraints(&circuit, true);
    let witness = r1cs.compute_witness(&circuit, inputs);

    println!("{:?}", witness);
}
