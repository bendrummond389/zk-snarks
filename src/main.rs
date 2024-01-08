#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
mod circuits;

use circuits::parser::parse_computation;
use circuits::r1cs::R1CS;
use circuits::{Circuit, Operand, Operation};

fn main() {
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
        Ok(circuit) => {
            let mut r1cs = R1CS::new();

            r1cs.traverse_and_index_circuit(&circuit, 0);
            r1cs.combine_variable_vectors();
            let vars = r1cs.combined_var_vector;
            println!("{:?}", vars)
        }
        Err(e) => println!("Failed to parse computation: {}", e),
    }

}
