mod circuits;
mod ecc;
mod zk_proof;

use circuits::parser::parse_circuit_from_file;
use circuits::r1cs::R1CS;
use ecc::finite_field::{generate_keys, point_operations};
use std::collections::HashMap;
use std::env;
use zk_proof::qap::QAP;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let file_path = "./src/sample_circuits/circuit2.json";

    generate_keys();
    point_operations();

    match parse_circuit_from_file(file_path) {
        Ok(circuit) => {
            let mut r1cs = R1CS::from_circuit(circuit);
            let mut inputs = HashMap::new();
            inputs.insert("1".to_string(), 1.0);
            inputs.insert("x".to_string(), 3.0);
            // inputs.insert("y".to_string(), 1.5);

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

            let mut qap = QAP::from_r1cs(&mut r1cs);
            let witness = qap.calculate_witness(inputs, r1cs);

            // println!("witness: {:?}", witness);
            // qap.calculate_dot_products();
            // qap.display_polynomials();
        }
        Err(e) => println!("Error: {}", e),
    }
}
