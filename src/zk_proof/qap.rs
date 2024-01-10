use super::polynomial::Polynomial;
use crate::circuits::r1cs::R1CS;

pub struct QAP {
    a_polynomials: Vec<Polynomial>,
    b_polynomials: Vec<Polynomial>,
    c_polynomials: Vec<Polynomial>,
    input_polynomial: Polynomial,
}

impl QAP {
    fn new() -> Self {
        QAP {
            a_polynomials: Vec::new(),
            b_polynomials: Vec::new(),
            c_polynomials: Vec::new(),
            input_polynomial: Polynomial::new(vec!1]),
        }
    }
    pub fn from_r1cs(r1cs: &R1CS) -> Self {
        // Conversion logic here
    }
}
