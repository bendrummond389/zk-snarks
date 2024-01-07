#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
mod circuits;

use circuits::parser::parse_computation;
use circuits::r1cs::R1CS;
use circuits::{Circuit, Operand, Operation};

fn explore_circuits(circuit: &Circuit) {
    println!("Operation: {:?}", circuit.operation);

    for operand in &circuit.operands {
        match operand {
            Operand::Number(num) => println!("Operand: Number({})", num),
            Operand::Variable(var) => println!("Operand: Variable({})", var),
            Operand::NestedCircuit(nested) => {
                println!("Operand: Nested Circuit");
                explore_circuits(nested);
            }
        }
    }
}

fn main() {
    // Example use (assuming `parse_computation` returns a Circuit)
    let json = r#"{
      "operation": "Add",
      "operands": [
          {"operation": "Multiply", "operands": ["x", "x"]},
          {"operation": "Multiply", "operands": ["x", 5]}
      ]
  }"#;

    match circuits::parser::parse_computation(json) {
        Ok(circuit) => {
            let mut r1cs = R1CS::new();
            r1cs.determine_circuit_size(&circuit);
            let length = r1cs.current_index;

            println!("Length: {}", length)
        }
        Err(e) => println!("Failed to parse computation: {}", e),
    }
}
