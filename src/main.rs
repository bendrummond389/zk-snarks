#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
mod circuits;
mod zk_proof;

use circuits::parser::parse_circuit_from_file;
use circuits::r1cs::R1CS;
use circuits::{Circuit, Operand, Operation};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::env;
use zk_proof::qap::QAP;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let file_path = "./src/sample_circuits/circuit1.json";

    match parse_circuit_from_file(file_path) {
        Ok(circuit) => {
            let mut r1cs = R1CS::from_circuit(circuit);

            // Get and print the matrices
            if let Some((a_matrix, b_matrix, c_matrix)) = r1cs.get_matrices() {
                println!("A Matrix:");
                for row in a_matrix {
                    println!("{:?}", row);
                }

                println!("B Matrix:");
                for row in b_matrix {
                    println!("{:?}", row);
                }

                println!("C Matrix:");
                for row in c_matrix {
                    println!("{:?}", row);
                }
            } else {
                println!("No matrices available.");
            }

            let qap = QAP::from_r1cs(&mut r1cs);
            // qap.display_polynomials();

            let dot = qap.calculate_dot_products();

            println!("a_poly{:?}", dot)

        }
        Err(e) => println!("Error: {}", e),
    }
}
