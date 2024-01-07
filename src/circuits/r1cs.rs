#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use super::Circuit;
use super::{Operand, Operation};
use std::collections::HashMap;

struct Constraint {
    a: Vec<i32>,
    b: Vec<i32>,
    c: Vec<i32>,
}

pub struct R1CS {
    a_matrix: Vec<Vec<i32>>,
    b_matrix: Vec<Vec<i32>>,
    c_matrix: Vec<Vec<i32>>,
    pub variable_indices: HashMap<String, usize>,
    pub current_index: usize,
}

impl R1CS {
    pub fn new() -> Self {
        R1CS {
            a_matrix: Vec::new(),
            b_matrix: Vec::new(),
            c_matrix: Vec::new(),
            variable_indices: HashMap::new(),
            current_index: 0,
        }
    }

    fn add_constraint(&mut self, constraint: Constraint) {
        self.a_matrix.push(constraint.a);
        self.b_matrix.push(constraint.b);
        self.c_matrix.push(constraint.c);
    }

    fn get_variable_index(&mut self, name: &str) -> usize {
        match self.variable_indices.get(name) {
            Some(&index) => index,
            None => {
                let new_index = self.current_index;
                self.variable_indices.insert(name.to_string(), new_index);
                new_index
            }
        }
    }

    pub fn determine_circuit_size(&mut self, circuit: &Circuit) {
        if circuit.operands.len() != 2 {
            panic!("Expected two operands for binary operation");
        }

        let operand1 = &circuit.operands[0];
        let operand2 = &circuit.operands[1];

        match (operand1, operand2) {
            (Operand::Number(num1), Operand::Number(num2)) => {
                println!("Multiplying {} and {}", num1, num2)
            }
            (Operand::Number(num1), Operand::Variable(num2))
            | (Operand::Variable(num2), Operand::Number(num1)) => {}
            (Operand::NestedCircuit(circuit), Operand::Number(num))
            | (Operand::Number(num), Operand::NestedCircuit(circuit)) => {}
            (Operand::NestedCircuit(circuit), Operand::Variable(var))
            | (Operand::Variable(var), Operand::NestedCircuit(circuit)) => {}
            (Operand::Variable(var1), Operand::Variable(var2)) => {}
            (Operand::NestedCircuit(circuit1), Operand::NestedCircuit(circuit2)) => {}
        }
    }

    pub fn process_circuit(&mut self, circuit: &Circuit) {
        match &circuit.operation {
            Operation::Add => {}
            Operation::Multiply => {}
        }
    }
}
