use thiserror::Error;

#[derive(Error, Debug)]
pub enum CircuitError {
    #[error("invalid operation")]
    InvalidOperation,

    #[error("operand mismatch: expected {0} operands, found {1}")]
    OperandMismatch(usize, usize),

    #[error("nested circuit error: {0}")]
    NestedCircuitError(String),

    #[error("evaluation error: {0}")]
    EvaluationError(String),

    #[error("file read error: {0}")]
    FileReadError(String),

    #[error("serialization error: {0}")]
    SerializationError(String),

    #[error("deserialization error: {0}")]
    DeserializationError(String),
}
