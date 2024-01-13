#[allow(dead_code)]
mod circuits;
// mod ecc;
mod ecc;
mod r1cs;
mod utils;
// mod zk_proof;

use circuits::Circuit;
use ecc::finite_field::point_operations;
use r1cs::r1cs::R1CS;
use std::{collections::HashMap, env};

// use crate::zk_proof::qap::QAP;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let file_path = "./circuits/sample_circuits/circuit2.json";

    let mut circuit = Circuit::from_file(file_path).expect("Failed to load circuit");
    let variable_map = circuit.hash_and_index_circuit();

    let mut r1cs = R1CS::new(variable_map);

    r1cs.generate_r1cs_constraints(&circuit, true);
    let (a_matrix, b_matrix, c_matrix) = r1cs.get_constraint_matrices();

    println!("A {:?}", a_matrix);
    println!("B {:?}", b_matrix);
    println!("C {:?}", c_matrix);

    println!("{:?}", circuit);
}
