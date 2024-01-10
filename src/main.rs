#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
mod circuits;
mod zk_proof;

use circuits::parser::parse_circuit_from_file;
use circuits::r1cs::R1CS;
use circuits::{Circuit, Operand, Operation};
use std::env;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let file_path = "./src/sample_circuits/circuit1.json";

    match circuits::parser::parse_circuit_from_file(file_path) {
        Ok(mut circuit) => {
            let r1cs = R1CS::from_circuit(&mut circuit);
            match r1cs.get_matrices() {
                Some((a_matrix, b_matrix, c_matrix)) => {
                    println!("A: {:?}", a_matrix);
                    println!("B: {:?}", b_matrix);
                    println!("C: {:?}", c_matrix);
                }
                None => println!("Empty matrices"),
            }
            match r1cs.get_witness() {
                Some(witness_vector) => {
                    println!("{:?}", witness_vector);
                }
                None => println!("No witness")
            }
        }
     
        Err(e) => println!("Error: {}", e),
    }
}
