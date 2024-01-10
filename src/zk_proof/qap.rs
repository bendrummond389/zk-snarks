use crate::circuits::r1cs::R1CS;
use polynomial::Polynomial;

pub struct QAP {
    a_polynomials: Vec<Polynomial<f64>>,
    b_polynomials: Vec<Polynomial<f64>>,
    c_polynomials: Vec<Polynomial<f64>>,
    input_polynomial: Polynomial<f64>,
}

impl QAP {
    pub fn from_r1cs(r1cs: &R1CS) -> Self {
        let (a_matrix, b_matrix, c_matrix) = match r1cs.get_matrices() {
            Some((a, b, c)) => (a.clone(), b.clone(), c.clone()),
            None => panic!("R1CS matrices are empty"),
        };

        let a_polynomials = generate_polynomials_from_matrix(&a_matrix);
        let b_polynomials = generate_polynomials_from_matrix(&b_matrix);
        let c_polynomials = generate_polynomials_from_matrix(&c_matrix);

        QAP {
            a_polynomials,
            b_polynomials,
            c_polynomials,
            input_polynomial: Polynomial::new(vec![1.0]),
        }
    }
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
