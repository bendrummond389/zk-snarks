use crate::circuits::r1cs::R1CS;
use polynomial::Polynomial;
use std::collections::HashMap;

pub struct QAP {
    a_polynomials: Vec<Polynomial<f64>>,
    b_polynomials: Vec<Polynomial<f64>>,
    c_polynomials: Vec<Polynomial<f64>>,
    witness: HashMap<String, f64>,
    variable_vector: Vec<String>,
}

impl QAP {
    pub fn from_r1cs(r1cs: &mut R1CS) -> Self {
        let (a_matrix, b_matrix, c_matrix) = r1cs.get_matrices().expect("R1CS matrices are empty");
        let variable_vector = r1cs.variable_vector.clone();

        QAP {
            a_polynomials: generate_polynomials_from_matrix(&a_matrix),
            b_polynomials: generate_polynomials_from_matrix(&b_matrix),
            c_polynomials: generate_polynomials_from_matrix(&c_matrix),
            witness: HashMap::new(),
            variable_vector,
        }
    }

    pub fn calculate_witness(&mut self, inputs: HashMap<String, f64>, mut r1cs: R1CS) {
        self.witness = r1cs.calculate_witness(inputs)
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

    pub fn calculate_dot_products(&self) {
        let a_result = calculate_dot_product_for_polynomials(
            &self.a_polynomials,
            &self.variable_vector,
            &self.witness,
        );

        let b_result = calculate_dot_product_for_polynomials(
            &self.b_polynomials,
            &self.variable_vector,
            &self.witness,
        );

        let c_result = calculate_dot_product_for_polynomials(
            &self.c_polynomials,
            &self.variable_vector,
            &self.witness,
        );
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
