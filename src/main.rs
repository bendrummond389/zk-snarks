#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
mod circuits;
mod zk_proof;

use circuits::parser::parse_circuit_from_file;
use circuits::r1cs::R1CS;
use circuits::{Circuit, Operand, Operation};
use std::env;
use zk_proof::qap::QAP;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let file_path = "./src/sample_circuits/circuit1.json";

    match circuits::parser::parse_circuit_from_file(file_path) {
        Ok(mut circuit) => {
            let r1cs = R1CS::from_circuit(&mut circuit);
            let qap = QAP::from_r1cs(&r1cs);
        }

        Err(e) => println!("Error: {}", e),
    }
}
