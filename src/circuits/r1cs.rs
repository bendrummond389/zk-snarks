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

#[derive(Debug)]
struct Constraint {
    a: Vec<i32>,
    b: Vec<i32>,
    c: Vec<i32>,
}

pub struct R1CS {
    pub a_matrix: Vec<Vec<i32>>,
    pub b_matrix: Vec<Vec<i32>>,
    pub c_matrix: Vec<Vec<i32>>,
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

    pub fn traverse_and_index_circuit(&mut self, circuit: &mut Circuit, depth: usize) -> u64 {
        // Binary operation check
        if circuit.operands.len() != 2 {
            panic!("Expected two operands for binary operation");
        }

        let mut hasher = DefaultHasher::new();

        // Hash the operation
        circuit.operation.hash(&mut hasher);

        for operand in &mut circuit.operands {
            match operand {
                Operand::Variable(var) => {
                    self.add_variable_to_inputs_vector(&var);
                    var.hash(&mut hasher);
                }
                Operand::NestedCircuit(nested_circuit) => {
                    let nested_hash = self.traverse_and_index_circuit(nested_circuit, depth + 1);
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

        circuit.hash = Some(circuit_hash);

        circuit_hash
    }

    pub fn generate_r1cs_constraints(&mut self, circuit: &Circuit, depth: usize) {
        let vector_degree = self.combined_var_vector.len();
        let circuit_hash = match circuit.hash {
            Some(hash) => hash,
            None => {
                panic!("Expected circuit to have hashes assigned")
            }
        };

        let circuit_index = if depth != 0 {
            match self.combined_indices.get(&circuit_hash.to_string()) {
                Some(&index) => index,
                None => panic!("Cannot find index of current circuit in combined_indices"),
            }
        } else {
            match self.combined_indices.get("out") {
                Some(&index) => index,
                None => panic!("Cannot find index of 'out' in combined_indices"),
            }
        };

        let mut constraint = Constraint {
            b: vec![0; vector_degree],
            c: vec![0; vector_degree],
            a: vec![0; vector_degree],
        };
        constraint.c[circuit_index] = 1;

        if circuit.operands.len() != 2 {
            panic!("Expected two operands for binary operation");
        }

        let operand1 = &circuit.operands[0];
        let operand2 = &circuit.operands[1];

        match (operand1, operand2) {
            // Number-Number Case
            (Operand::Number(num1), Operand::Number(num2)) => match &circuit.operation {
                Operation::Add => {
                    constraint.a[0] = num1 + num2;
                    constraint.b[0] = 1;
                    constraint.c[circuit_index] = 1;
                }
                Operation::Multiply => {
                    constraint.a[0] = *num1;
                    constraint.b[0] = *num2;
                    constraint.c[circuit_index] = 1;
                }
            },

            // Number-Variable and Variable-Number Cases
            (Operand::Number(num), Operand::Variable(var))
            | (Operand::Variable(var), Operand::Number(num)) => {
                let var_index = match self.combined_indices.get(var) {
                    Some(&index) => index,
                    None => panic!("Cannot find index of variable in combined_indices"),
                };

                match &circuit.operation {
                    Operation::Add => {
                        constraint.a[0] = *num;
                        constraint.a[var_index] = 1;
                        constraint.b[0] = 1;
                    }
                    Operation::Multiply => {
                        constraint.a[0] = *num;
                        constraint.b[var_index] = 1;
                    }
                }
            }

            // Variable-Variable Case
            (Operand::Variable(var1), Operand::Variable(var2)) => {
                let var1_index = match self.combined_indices.get(var1) {
                    Some(&index) => index,
                    None => panic!("Cannot find index of variable in combined_indices"),
                };
                let var2_index = match self.combined_indices.get(var2) {
                    Some(&index) => index,
                    None => panic!("Cannot find index of variable in combined_indices"),
                };

                match &circuit.operation {
                    Operation::Add => {
                        if var1 == var2 {
                            constraint.a[var1_index] = 2;
                            constraint.b[0] = 1;
                        } else {
                            constraint.a[var1_index] = 1;
                            constraint.a[var2_index] = 1;
                            constraint.b[0] = 1;
                        }
                    }
                    Operation::Multiply => {
                        constraint.a[var1_index] = 1;
                        constraint.b[var2_index] = 1;
                    }
                }
            }

            // Nested Circuit-Number and Number-Nested Circuit Cases
            (Operand::NestedCircuit(nested_circuit), Operand::Number(num))
            | (Operand::Number(num), Operand::NestedCircuit(nested_circuit)) => {
                let nested_circuit_hash = match nested_circuit.hash {
                    Some(hash) => hash,
                    None => {
                        panic!("Expected circuit to have hashes assigned")
                    }
                };

                let nested_circuit_index =
                    match self.combined_indices.get(&nested_circuit_hash.to_string()) {
                        Some(&index) => index,
                        None => {
                            panic!("Expected circuit to have hashes assigned")
                        }
                    };

                match &circuit.operation {
                    Operation::Add => {
                        constraint.a[0] = *num;
                        constraint.a[nested_circuit_index] = 1;
                        constraint.b[0] = 1;
                    }
                    Operation::Multiply => {
                        constraint.a[0] = *num;
                        constraint.b[nested_circuit_index] = 1;
                    }
                }

                self.generate_r1cs_constraints(&nested_circuit, depth + 1);
            }

            // Nested Circuit-Variable and Variable-Nested Circuit Cases
            (Operand::NestedCircuit(nested_circuit), Operand::Variable(var))
            | (Operand::Variable(var), Operand::NestedCircuit(nested_circuit)) => {
                let nested_circuit_hash = match nested_circuit.hash {
                    Some(hash) => hash,
                    None => {
                        panic!("Expected circuit to have hashes assigned")
                    }
                };

                let nested_circuit_index =
                    match self.combined_indices.get(&nested_circuit_hash.to_string()) {
                        Some(&index) => index,
                        None => {
                            panic!("Expected circuit to have hashes assigned")
                        }
                    };

                let var_index = match self.combined_indices.get(var) {
                    Some(&index) => index,
                    None => panic!("Cannot find index of variable in combined_indices"),
                };

                match &circuit.operation {
                    Operation::Add => {
                        constraint.a[nested_circuit_index] = 1;
                        constraint.a[var_index] = 1;
                        constraint.b[0] = 1
                    }
                    Operation::Multiply => {
                        constraint.a[nested_circuit_index] = 1;
                        constraint.b[var_index] = 1;
                    }
                }

                self.generate_r1cs_constraints(&nested_circuit, depth + 1);
            }

            // Nested Circuit-Nested Circuit Case
            (Operand::NestedCircuit(nested_circuit1), Operand::NestedCircuit(nested_circuit2)) => {
                let nested_circuit_hash1 = match nested_circuit1.hash {
                    Some(hash) => hash,
                    None => {
                        panic!("Expected circuit to have hashes assigned")
                    }
                };

                let nested_circuit_hash2 = match nested_circuit2.hash {
                    Some(hash) => hash,
                    None => {
                        panic!("Expected circuit to have hashes assigned")
                    }
                };

                let nested_circuit_index1 =
                    match self.combined_indices.get(&nested_circuit_hash1.to_string()) {
                        Some(&index) => index,
                        None => {
                            panic!("Expected circuit to have hashes assigned")
                        }
                    };

                let nested_circuit_index2 =
                    match self.combined_indices.get(&nested_circuit_hash2.to_string()) {
                        Some(&index) => index,
                        None => {
                            panic!("Expected circuit to have hashes assigned")
                        }
                    };

                match &circuit.operation {
                    Operation::Add => {
                        if nested_circuit_hash1 == nested_circuit_hash2 {
                            constraint.a[nested_circuit_index1] = 2;
                            constraint.b[0] = 1
                        } else {
                            constraint.a[nested_circuit_index1] = 1;
                            constraint.a[nested_circuit_index2] = 1;
                            constraint.b[0] = 1;
                        }
                    }
                    Operation::Multiply => {
                        constraint.a[nested_circuit_index1] = 1;
                        constraint.b[nested_circuit_index2] = 1;
                    }
                }

                self.generate_r1cs_constraints(&nested_circuit1, depth + 1);
                self.generate_r1cs_constraints(&nested_circuit2, depth + 1);
            }
        }
        self.add_constraint(constraint);
    }
}
