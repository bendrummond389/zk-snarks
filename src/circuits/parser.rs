#[allow(dead_code)]
use super::Circuit;
use serde_json::Error;
use std::fs;

pub fn parse_circuit(input: &str) -> Result<Circuit, Error> {
    serde_json::from_str(input)
}

pub fn parse_circuit_from_file(file_path: &str) -> Result<Circuit, String> {
    match fs::read_to_string(file_path) {
        Ok(json) => parse_circuit(&json).map_err(|e| e.to_string()),
        Err(e) => Err(format!("Failed to read file: {}", e)),
    }
}
