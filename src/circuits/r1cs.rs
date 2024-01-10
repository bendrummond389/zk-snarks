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
    a_matrix: Vec<Vec<i32>>,
    b_matrix: Vec<Vec<i32>>,
    c_matrix: Vec<Vec<i32>>,
    witness_vector: Vec<String>,
    witness_indices: HashMap<String, usize>,
}

impl R1CS {
    fn new() -> Self {
        let mut r1cs = R1CS {
            a_matrix: Vec::new(),
            b_matrix: Vec::new(),
            c_matrix: Vec::new(),
            witness_vector: Vec::new(),
            witness_indices: HashMap::new(),
        };

        r1cs
    }

    pub fn from_circuit(circuit: &mut Circuit) -> Self {
        let mut r1cs = R1CS::new();

        let mut static_variables = vec!["1".to_string()];
        let mut linearization_variables = Vec::new();

        let mut static_var_indices: HashMap<String, usize> = HashMap::new();
        let mut linearization_var_indices: HashMap<String, usize> = HashMap::new();

        r1cs.traverse_and_index_circuit(
            circuit,
            0,
            &mut static_variables,
            &mut static_var_indices,
            &mut linearization_variables,
            &mut linearization_var_indices,
        );

        // Combine static and linearization vectors to create our witness vector
        r1cs.combine_variable_vectors(&mut static_variables, &mut linearization_variables);

        r1cs.generate_r1cs_constraints(circuit, 0);
        r1cs
    }

    fn add_constraint(&mut self, constraint: Constraint) {
        self.a_matrix.push(constraint.a);
        self.b_matrix.push(constraint.b);
        self.c_matrix.push(constraint.c);
    }

    pub fn get_matrices(&self) -> Option<(&Vec<Vec<i32>>, &Vec<Vec<i32>>, &Vec<Vec<i32>>)> {
        if self.a_matrix.is_empty() || self.b_matrix.is_empty() || self.c_matrix.is_empty() {
            None
        } else {
            Some((&self.a_matrix, &self.b_matrix, &self.c_matrix))
        }
    }

    fn add_variable_to_static_vector(
        &mut self,
        var: &str,
        static_variables: &mut Vec<String>,
        static_var_indices: &mut HashMap<String, usize>,
    ) -> usize {
        match static_var_indices.get(var) {
            Some(&index) => index,
            None => {
                static_variables.push(var.to_string());
                let new_index = static_var_indices.len();
                static_var_indices.insert(var.to_string(), new_index);
                new_index
            }
        }
    }

    fn add_variable_to_linearization_vector(
        &mut self,
        circuit_hash: u64,
        linearization_variables: &mut Vec<String>,
        linearization_var_indices: &mut HashMap<String, usize>,
    ) -> usize {
        let hash_str = circuit_hash.to_string();
        match linearization_var_indices.get(&hash_str) {
            Some(&index) => index,
            None => {
                let new_index = linearization_var_indices.len();
                linearization_var_indices.insert(hash_str.clone(), new_index);
                linearization_variables.push(hash_str);
                new_index
            }
        }
    }

    pub fn combine_variable_vectors(
        &mut self,
        static_variables: &mut Vec<String>,
        linearization_variables: &mut Vec<String>,
    ) {
        let mut combined_vars = static_variables.clone();
        combined_vars.extend(linearization_variables.clone());
        self.witness_vector = combined_vars.clone();

        let index_map: HashMap<String, usize> = combined_vars
            .into_iter()
            .enumerate()
            .map(|(index, var)| (var, index))
            .collect();

        self.witness_indices = index_map
    }

    pub fn traverse_and_index_circuit(
        &mut self,
        circuit: &mut Circuit,
        depth: usize,
        static_variables: &mut Vec<String>,
        static_var_indices: &mut HashMap<String, usize>,
        linearization_variables: &mut Vec<String>,
        linearization_var_indices: &mut HashMap<String, usize>,
    ) -> u64 {
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
                    self.add_variable_to_static_vector(&var, static_variables, static_var_indices);
                    var.hash(&mut hasher);
                }
                Operand::NestedCircuit(nested_circuit) => {
                    let nested_hash = self.traverse_and_index_circuit(
                        nested_circuit,
                        depth + 1,
                        static_variables,
                        static_var_indices,
                        linearization_variables,
                        linearization_var_indices,
                    );
                    hasher.write_u64(nested_hash);
                }
                Operand::Number(num) => num.hash(&mut hasher),
            }
        }

        let circuit_hash = hasher.finish();
        if depth != 0 {
            self.add_variable_to_linearization_vector(
                circuit_hash,
                linearization_variables,
                linearization_var_indices,
            );
        } else {
            self.add_variable_to_static_vector("out", static_variables, static_var_indices);
        }

        circuit.hash = Some(circuit_hash);

        circuit_hash
    }

    pub fn generate_r1cs_constraints(&mut self, circuit: &Circuit, depth: usize) {
        let vector_degree = self.witness_vector.len();
        let circuit_hash = match circuit.hash {
            Some(hash) => hash,
            None => {
                panic!("Expected circuit to have hashes assigned")
            }
        };

        let circuit_index = if depth != 0 {
            match self.witness_indices.get(&circuit_hash.to_string()) {
                Some(&index) => index,
                None => panic!("Cannot find index of current circuit in witness_indices"),
            }
        } else {
            match self.witness_indices.get("out") {
                Some(&index) => index,
                None => panic!("Cannot find index of 'out' in witness_indices"),
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
                let var_index = match self.witness_indices.get(var) {
                    Some(&index) => index,
                    None => panic!("Cannot find index of variable in witness_indices"),
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
                let var1_index = match self.witness_indices.get(var1) {
                    Some(&index) => index,
                    None => panic!("Cannot find index of variable in witness_indices"),
                };
                let var2_index = match self.witness_indices.get(var2) {
                    Some(&index) => index,
                    None => panic!("Cannot find index of variable in witness_indices"),
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
                    match self.witness_indices.get(&nested_circuit_hash.to_string()) {
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
                    match self.witness_indices.get(&nested_circuit_hash.to_string()) {
                        Some(&index) => index,
                        None => {
                            panic!("Expected circuit to have hashes assigned")
                        }
                    };

                let var_index = match self.witness_indices.get(var) {
                    Some(&index) => index,
                    None => panic!("Cannot find index of variable in witness_indices"),
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
                    match self.witness_indices.get(&nested_circuit_hash1.to_string()) {
                        Some(&index) => index,
                        None => {
                            panic!("Expected circuit to have hashes assigned")
                        }
                    };

                let nested_circuit_index2 =
                    match self.witness_indices.get(&nested_circuit_hash2.to_string()) {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_simple_addition_circuit() -> Circuit {
        Circuit {
            operation: Operation::Add,
            operands: vec![Operand::Number(1), Operand::Number(2)],
            hash: None,
        }
    }

    fn setup_simple_multiplication_circuit() -> Circuit {
        Circuit {
            operation: Operation::Multiply,
            operands: vec![Operand::Number(3), Operand::Number(4)],
            hash: None,
        }
    }

    fn setup_variable_addition_circuit() -> Circuit {
        Circuit {
            operation: Operation::Add,
            operands: vec![
                Operand::Variable("x".to_string()),
                Operand::Variable("y".to_string()),
            ],
            hash: None,
        }
    }

    fn setup_variable_multiplication_circuit() -> Circuit {
        Circuit {
            operation: Operation::Multiply,
            operands: vec![
                Operand::Variable("x".to_string()),
                Operand::Variable("y".to_string()),
            ],
            hash: None,
        }
    }

    fn setup_nested_addition_circuit() -> Circuit {
        Circuit {
            operation: Operation::Add,
            operands: vec![
                Operand::NestedCircuit(Box::new(Circuit {
                    operation: Operation::Add,
                    operands: vec![Operand::Number(1), Operand::Number(2)],
                    hash: None,
                })),
                Operand::Number(5),
            ],
            hash: None,
        }
    }

    fn setup_nested_multiplication_circuit() -> Circuit {
        Circuit {
            operation: Operation::Add,
            operands: vec![
                Operand::NestedCircuit(Box::new(Circuit {
                    operation: Operation::Multiply,
                    operands: vec![Operand::Number(1), Operand::Number(2)],
                    hash: None,
                })),
                Operand::Number(5),
            ],
            hash: None,
        }
    }

    #[test]
    fn test_simple_addition_circuit_r1cs() {
        let mut circuit = setup_simple_addition_circuit();
        let r1cs = R1CS::from_circuit(&mut circuit);

        let (a_matrix, b_matrix, c_matrix) =
            r1cs.get_matrices().expect("Matrices should not be empty");

        assert_eq!(a_matrix, &vec![vec![3, 0]]);
        assert_eq!(b_matrix, &vec![vec![1, 0]]);
        assert_eq!(c_matrix, &vec![vec![0, 1]]);
    }

    #[test]
    fn test_simple_multiplication_circuit_r1cs() {
        let mut circuit = setup_simple_multiplication_circuit();
        let r1cs = R1CS::from_circuit(&mut circuit);

        let (a_matrix, b_matrix, c_matrix) =
            r1cs.get_matrices().expect("Matrices should not be empty");

        assert_eq!(a_matrix, &vec![vec![3, 0]]);
        assert_eq!(b_matrix, &vec![vec![4, 0]]);
        assert_eq!(c_matrix, &vec![vec![0, 1]]);
    }

    #[test]
    fn test_variable_addition_circuit_r1cs() {
        let mut circuit = setup_variable_addition_circuit();
        let r1cs = R1CS::from_circuit(&mut circuit);

        let (a_matrix, b_matrix, c_matrix) =
            r1cs.get_matrices().expect("Matrices should not be empty");

        assert_eq!(a_matrix, &vec![vec![0, 1, 1, 0]]);
        assert_eq!(b_matrix, &vec![vec![1, 0, 0, 0]]);
        assert_eq!(c_matrix, &vec![vec![0, 0, 0, 1]]);
    }

    #[test]
    fn test_variable_multiplication_circuit_r1cs() {
        let mut circuit = setup_variable_multiplication_circuit();
        let r1cs = R1CS::from_circuit(&mut circuit);

        let (a_matrix, b_matrix, c_matrix) =
            r1cs.get_matrices().expect("Matrices should not be empty");

        assert_eq!(a_matrix, &vec![vec![0, 1, 0, 0]]);
        assert_eq!(b_matrix, &vec![vec![0, 0, 1, 0]]);
        assert_eq!(c_matrix, &vec![vec![0, 0, 0, 1]]);
    }

    #[test]
    fn test_nested_addition_circuit_r1cs() {
        let mut circuit = setup_nested_addition_circuit();
        let r1cs = R1CS::from_circuit(&mut circuit);

        let (a_matrix, b_matrix, c_matrix) =
            r1cs.get_matrices().expect("Matrices should not be empty");

        assert_eq!(a_matrix, &vec![vec![3, 0, 0], vec![5, 0, 1]]);
        assert_eq!(b_matrix, &vec![vec![1, 0, 0], vec![1, 0, 0]]);
        assert_eq!(c_matrix, &vec![vec![0, 0, 1], vec![0, 1, 0]]);
    }

    #[test]
    fn test_nested_multiplication_circuit_r1cs() {
        let mut circuit = setup_nested_multiplication_circuit();
        let r1cs = R1CS::from_circuit(&mut circuit);

        let (a_matrix, b_matrix, c_matrix) =
            r1cs.get_matrices().expect("Matrices should not be empty");

        assert_eq!(a_matrix, &vec![vec![1, 0, 0], vec![5, 0, 1]]);
        assert_eq!(b_matrix, &vec![vec![2, 0, 0], vec![1, 0, 0]]);
        assert_eq!(c_matrix, &vec![vec![0, 0, 1], vec![0, 1, 0]]);
    }
}
