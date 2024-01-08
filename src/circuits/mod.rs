#[allow(dead_code)]
use serde::{Deserialize, Serialize};

mod engine;
pub mod parser;
pub mod r1cs;

#[derive(Serialize, Deserialize, Debug)]
#[derive(Hash)]
pub enum Operation {
    Add,
    Multiply,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Circuit {
    pub operation: Operation, // remove pub after testing
    pub operands: Vec<Operand>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Operand {
    Number(i32),
    Variable(String),  
    NestedCircuit(Box<Circuit>),
}
