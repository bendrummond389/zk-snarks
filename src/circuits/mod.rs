#[allow(dead_code)]
use serde::{Deserialize, Serialize};

pub mod parser;
pub mod r1cs;

#[derive(Serialize, Deserialize, Debug, Hash, Clone)]
pub enum Operation {
    Add,
    Multiply,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Circuit {
    pub operation: Operation,
    pub operands: Vec<Operand>,
    pub hash: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Operand {
    Number(f64),
    Variable(String),
    NestedCircuit(Box<Circuit>),
}
