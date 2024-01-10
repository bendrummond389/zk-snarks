#[allow(dead_code)]
use serde::{Deserialize, Serialize};

pub mod parser;
pub mod r1cs;

#[derive(Serialize, Deserialize, Debug, Hash)]
pub enum Operation {
    Add,
    Multiply,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Circuit {
    pub operation: Operation,
    pub operands: Vec<Operand>,
    pub hash: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Operand {
    Number(i32),
    Variable(String),
    NestedCircuit(Box<Circuit>),
}
