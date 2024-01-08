#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]

use super::Circuit;
use super::{Operand, Operation};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::usize;


struct Constraint {
    a: Vec<i32>,
    b: Vec<i32>,
    c: Vec<i32>,
}

pub struct R1CS {
    a_matrix: Vec<Vec<i32>>,
    b_matrix: Vec<Vec<i32>>,
    c_matrix: Vec<Vec<i32>>,
    pub static_variables: Vec<String>,
    pub linearization_variables: Vec<String>,
    pub combined_var_vector: Vec<String>,
    pub static_var_indices: HashMap<String, usize>,
    pub linearization_var_indices: HashMap<String, usize>,
    pub combined_indices: HashMap<String, usize>,
}

impl R1CS {
    pub fn new() -> Self {
        let mut r1cs = R1CS {
            a_matrix: Vec::new(),
            b_matrix: Vec::new(),
            c_matrix: Vec::new(),
            static_variables: Vec::new(),
            linearization_variables: Vec::new(),
            combined_var_vector: Vec::new(),
            static_var_indices: HashMap::new(),
            linearization_var_indices: HashMap::new(),
            combined_indices: HashMap::new(),
        };

        r1cs.static_variables.push("1".to_string());
        r1cs
    }

    fn add_constraint(&mut self, constraint: Constraint) {
        self.a_matrix.push(constraint.a);
        self.b_matrix.push(constraint.b);
        self.c_matrix.push(constraint.c);
    }

    fn get_variable_index(&mut self, name: &str) -> usize {
        match self.static_var_indices.get(name) {
            Some(&index) => index,
            None => {
                let new_index = self.static_var_indices.len();
                self.static_var_indices.insert(name.to_string(), new_index);
                new_index
            }
        }
    }

    fn add_variable_to_inputs_vector(&mut self, name: &str) -> usize {
        match self.static_var_indices.get(name) {
            Some(&index) => index,
            None => {
                self.static_variables.push(name.to_string());
                let new_index = self.static_var_indices.len();
                self.static_var_indices.insert(name.to_string(), new_index);
                new_index
            }
        }
    }

    fn add_variable_to_operation_vector(&mut self, operation_hash: u64) -> usize {
        let hash_str = operation_hash.to_string();
        match self.linearization_var_indices.get(&hash_str) {
            Some(&index) => index,
            None => {
                let new_index = self.linearization_var_indices.len();
                self.linearization_var_indices
                    .insert(hash_str.clone(), new_index);
                self.linearization_variables.push(hash_str);
                new_index
            }
        }
    }

    pub fn combine_variable_vectors(&mut self) {
        let mut combined_vars = self.static_variables.clone();
        combined_vars.extend(self.linearization_variables.clone());
        self.combined_var_vector = combined_vars.clone();

        let index_map: HashMap<String, usize> = combined_vars
            .into_iter()
            .enumerate()
            .map(|(index, var)| (var, index))
            .collect();

        self.combined_indices = index_map
    }

    pub fn traverse_and_index_circuit(&mut self, circuit: &Circuit, depth: usize) -> u64 {
        println!("Depth: {}", depth);
        // Binary operation check
        if circuit.operands.len() != 2 {
            panic!("Expected two operands for binary operation");
        }

        let mut hasher = DefaultHasher::new();

        // Hash the operation
        circuit.operation.hash(&mut hasher);

        for operand in &circuit.operands {
            match operand {
                Operand::Variable(var) => {
                    self.add_variable_to_inputs_vector(var);
                    let index = self.get_variable_index(&var);
                    hasher.write_usize(index)
                }
                Operand::NestedCircuit(circuit) => {
                    let nested_hash = self.traverse_and_index_circuit(circuit, depth + 1);
                    hasher.write_u64(nested_hash);
                }
                Operand::Number(num) => num.hash(&mut hasher),
            }
        }

        let circuit_hash = hasher.finish();
        if depth != 0 {
            self.add_variable_to_operation_vector(circuit_hash);
        } else {
            self.add_variable_to_inputs_vector("out");
        }

        circuit_hash
    }


}
