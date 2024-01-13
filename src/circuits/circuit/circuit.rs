use crate::circuits::errors::CircuitError;
use crate::circuits::indexed_map::IndexedMap;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::{Hash, Hasher};

#[derive(Serialize, Deserialize, Debug, Clone, Hash)]
pub enum Operation {
    Add,
    Multiply,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Operand {
    Number(i64),
    Variable(String),
    NestedCircuit(Box<Circuit>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Circuit {
    pub operation: Operation,
    pub operands: Vec<Operand>,
    #[serde(default)]
    pub hash: u64,
}

impl Circuit {
    pub fn new(
        operation: Operation,
        operand1: Operand,
        operand2: Operand,
        hash: Option<u64>,
    ) -> Self {
        Circuit {
            operation,
            operands: vec![operand1, operand2],
            hash: hash.unwrap_or(0),
        }
    }

    pub fn from_file(file_path: &str) -> Result<Self, CircuitError> {
        let json = fs::read_to_string(file_path)
            .map_err(|e| CircuitError::FileReadError(e.to_string()))?;

        serde_json::from_str(&json).map_err(|e| CircuitError::DeserializationError(e.to_string()))
    }

    pub fn set_hash(&mut self, new_hash: u64) {
        self.hash = new_hash;
    }

    pub fn get_hash(&self) -> u64 {
        self.hash
    }

    pub fn is_valid(&self) -> bool {
        self.operands.len() == 2
    }

    /// Hashes and indexes the circuit, combining static and linearization variables.
    pub fn hash_and_index_circuit(&mut self) -> IndexedMap<String> {
        // Initialize linearization and static variables
        let mut linearization_variables = IndexedMap::new();
        let mut static_variables = IndexedMap::from_vector(vec!["1".to_string()]);

        // Recursive hashing and indexing
        self.hash_and_index_recursive(true, &mut static_variables, &mut linearization_variables);

        // Merge static and linearization variables into one IndexedMap
        let mut combined_variables = static_variables.into_vector();
        combined_variables.extend(linearization_variables.into_vector());
        IndexedMap::from_vector(combined_variables)
    }

    fn hash_and_index_recursive(
        &mut self,
        root: bool,
        static_variables: &mut IndexedMap<String>,
        linearization_variables: &mut IndexedMap<String>,
    ) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.operation.hash(&mut hasher);

        for operand in &mut self.operands {
            match operand {
                Operand::Variable(var) => {
                    static_variables.add(var.clone());
                    var.hash(&mut hasher);
                }
                Operand::NestedCircuit(nested_circuit) => {
                    let nested_hash = nested_circuit.hash_and_index_recursive(
                        false,
                        static_variables,
                        linearization_variables,
                    );
                    hasher.write_u64(nested_hash);
                }
                Operand::Number(num) => num.hash(&mut hasher),
            }
        }

        let circuit_hash = hasher.finish();

        self.set_hash(circuit_hash);
        if root {
            static_variables.add("out".to_string());
        } else {
            linearization_variables.add(circuit_hash.to_string());
        }

        circuit_hash
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_circuit() {
        let operand1 = Operand::Number(1);
        let operand2 = Operand::Number(2);
        let circuit = Circuit::new(Operation::Add, operand1, operand2, None);

        assert_eq!(circuit.operands.len(), 2);
        assert_eq!(circuit.hash, 0);
    }
}
