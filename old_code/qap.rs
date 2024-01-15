use crate::{circuits::IndexedMap, r1cs::r1cs::R1CS};
use polynomial::Polynomial;
use std::{collections::HashMap, env::var};

pub struct QAP {
    a_polynomials: Vec<Polynomial<f64>>,
    b_polynomials: Vec<Polynomial<f64>>,
    c_polynomials: Vec<Polynomial<f64>>,
    witness: HashMap<String, f64>,
    variable_map: IndexedMap<String>,
}

impl QAP {
    pub fn from_r1cs(r1cs: R1CS, witness: HashMap<String, f64>) -> Self {
        let (a_matrix, b_matrix, c_matrix) = r1cs.get_constraint_matrices();
        let variable_map = r1cs.get_variable_map();

        QAP {
            a_polynomials: generate_polynomials_from_matrix(a_matrix),
            b_polynomials: generate_polynomials_from_matrix(b_matrix),
            c_polynomials: generate_polynomials_from_matrix(c_matrix),
            witness,
            variable_map: variable_map.clone(),
        }
    }

    pub fn calculate_dot_products(&self) -> (Polynomial<f64>, Polynomial<f64>, Polynomial<f64>) {
        let variable_vector = self.variable_map.clone().into_vector();
        (
            calculate_dot_product_for_polynomials(
                &self.a_polynomials,
                &variable_vector,
                &self.witness,
            ),
            calculate_dot_product_for_polynomials(
                &self.b_polynomials,
                &variable_vector,
                &self.witness,
            ),
            calculate_dot_product_for_polynomials(
                &self.c_polynomials,
                &variable_vector,
                &self.witness,
            ),
        )
    }

    pub fn display_polynomials(&self) {
        self.display_polynomial_set(&self.a_polynomials, "A");
        self.display_polynomial_set(&self.b_polynomials, "B");
        self.display_polynomial_set(&self.c_polynomials, "C");
    }

    fn display_polynomial_set(&self, polynomials: &[Polynomial<f64>], label: &str) {
        println!("{} Polynomials:", label);
        for poly in polynomials {
            println!("{:?}", poly);
        }
    }
}

fn calculate_dot_product_for_polynomials(
    polynomials: &[Polynomial<f64>],
    variable_vector: &[String],
    witness: &HashMap<String, f64>,
) -> Polynomial<f64> {
    let max_length = polynomials
        .iter()
        .map(|p| p.data().len())
        .max()
        .unwrap_or(0);
    let mut result_coeffs = vec![0.0; max_length];

    for (i, polynomial) in polynomials.iter().enumerate() {
        let coefficient = *witness
            .get(&variable_vector[i])
            .expect("Variable value missing in witness");

        for (j, &poly_coeff) in polynomial.data().iter().enumerate() {
            if j < result_coeffs.len() {
                result_coeffs[j] += poly_coeff * coefficient;
            } else {
                result_coeffs.push(poly_coeff * coefficient);
            }
        }
    }
    Polynomial::new(result_coeffs)
}

fn generate_polynomials_from_matrix(matrix: &Vec<Vec<f64>>) -> Vec<Polynomial<f64>> {
    let mut polynomials: Vec<Polynomial<f64>> = Vec::new();
    let num_columns = matrix[0].len();

    for col in 0..num_columns {
        let mut xs = Vec::new();
        let mut ys = Vec::new();

        for (row_index, row) in matrix.iter().enumerate() {
            let val = *row.get(col).unwrap_or(&0.0);
            xs.push(row_index as f64 + 1.0);
            ys.push(val);
        }

        if ys.iter().all(|&val| val == 0.0) {
            polynomials.push(Polynomial::new(vec![0.0]));
            continue;
        }

        if let Some(poly) = Polynomial::lagrange(&xs, &ys) {
            polynomials.push(poly);
        } else {
            panic!("Failed to create Lagrange polynomial for column {}", col);
        }
    }

    polynomials
}
