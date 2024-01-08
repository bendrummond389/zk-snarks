#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
mod circuits;

use circuits::parser::parse_computation;
use circuits::r1cs::R1CS;
use circuits::{Circuit, Operand, Operation};
use std::env;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let json = r#"{
        "operation": "Multiply",
        "operands": [
            {
                "operation": "Add",
                "operands": [
                    {
                        "operation": "Multiply",
                        "operands": ["x", "x"]
                    },
                    {
                        "operation": "Add",
                        "operands": ["x", 2]
                    }
                ]
            },
            {
                "operation": "Multiply",
                "operands": [
                    {
                        "operation": "Add",
                        "operands": ["x", 3]
                    },
                    4
                ]
            }
        ]
    }"#;

    match circuits::parser::parse_computation(json) {
        Ok(mut circuit) => {
            let mut r1cs = R1CS::new();

            r1cs.traverse_and_index_circuit(&mut circuit, 0);
            r1cs.combine_variable_vectors();
            r1cs.generate_r1cs_constraints(&circuit, 0);
            let vars = r1cs.combined_var_vector;
            let a = r1cs.a_matrix;
            let b = r1cs.b_matrix;
            let c = r1cs.c_matrix;
            println!("{:?}", vars);

            println!("A: {:?}", a);
            println!("B: {:?}", b);
            println!("C: {:?}", c);
        }
        Err(e) => println!("Failed to parse computation: {}", e),
    }
}
