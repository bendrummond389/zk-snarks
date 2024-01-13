#[allow(dead_code)]
mod circuits;
// mod ecc;
mod ecc;
mod r1cs;
mod utils;
mod zk_proof;

use circuits::Circuit;
use ecc::finite_field::point_operations;
use r1cs::r1cs::R1CS;
use std::{collections::HashMap, env};

use crate::zk_proof::qap::QAP;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let file_path = "./circuits/sample_circuits/circuit1.json";

    let mut circuit = Circuit::from_file(file_path).expect("Failed to load circuit");
    let variable_map = circuit.hash_and_index_circuit();

    let mut r1cs = R1CS::new(variable_map);

    r1cs.generate_r1cs_constraints(&circuit, true);
    let (a_matrix, b_matrix, c_matrix) = r1cs.get_constraint_matrices();

    let mut inputs = HashMap::new();

    inputs.insert("1".to_string(), 1.0);
    inputs.insert("x".to_string(), 3.0);

    let witness = r1cs.compute_witness(&circuit, inputs).clone();
    let variable_vector = r1cs.variable_map.clone().into_vector();

    println!("{:?}", variable_vector);
    println!("{:?}", witness);

    // println!("A {:?}", a_matrix);
    // println!("B {:?}", b_matrix);
    // println!("C {:?}", c_matrix);

    let qap = QAP::from_r1cs(r1cs, witness);
    // qap.display_polynomials();

    let (a_poly, b_poly, c_poly) = qap.calculate_dot_products();

    println!("A Polynomial: {:?}", a_poly);
    println!("B Polynomial: {:?}", b_poly);
    println!("C Polynomial: {:?}", c_poly);

    point_operations();
}
