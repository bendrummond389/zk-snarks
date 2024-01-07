#[allow(dead_code)]
use super::Circuit;
use serde_json::Error;

pub fn parse_computation(input: &str) -> Result<Circuit, Error> {
    serde_json::from_str(input)
}

// Example schema:
// {
//   "operation": "add",
//   "operands": [5, 3]
// }

// For nested operations:
// {
//   "operation": "Add",
//   "operands": [
//       {"operation": "Multiply", "operands": ["x", "x"]},
//       {"operation": "Multiply", "operands": ["x", 5]}
//   ]
// }