use crate::{circuits::IndexedMap, Polynomial};
use std::collections::HashMap;

pub struct QAP {
    a_polynomials: Vec<Polynomial>,
    b_polynomials: Vec<Polynomial>,
    c_polynomials: Vec<Polynomial>,
    witness: HashMap<String, i64>,
    variable_map: IndexedMap<String>,
}


impl QAP {
  
}