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

        let mut a_polynomials: Vec<Polynomial<f64>> = Vec::new();
        let num_columns = a_matrix[0].len();

        for col in 0..num_columns {
            let mut xs = Vec::new();
            let mut ys = Vec::new();

            for (row_index, row) in a_matrix.iter().enumerate() {
                let val = *row.get(col).unwrap_or(&0) as f64;
                xs.push(row_index as f64 + 1.0);
                ys.push(val);
            }

            if ys.iter().all(|&val| val == 0.0) {
                a_polynomials.push(Polynomial::new(vec![0.0]));
                continue;
            }

            if let Some(poly) = Polynomial::lagrange(&xs, &ys) {
                a_polynomials.push(poly);
            } else {
                panic!("Failed to create Lagrange polynomial for column {}", col);
            }
        }

        let mut a_values_at_1: Vec<f64> = Vec::new();
        for poly in &a_polynomials {
            let value_at_1 = poly.eval(4.0);
            let rounded_value_at_1 = (value_at_1 * 100.0).round() / 100.0;
            a_values_at_1.push(rounded_value_at_1);
        }

        println!("{:?}", a_matrix[3]);

        println!("A evaluated at x = 4: {:?}", a_values_at_1);

        QAP {
            a_polynomials,
            b_polynomials: Vec::new(), // TODO: populate this similarly
            c_polynomials: Vec::new(), // TODO: populate this similarly
            input_polynomial: Polynomial::new(vec![1.0]), // Adjusted for f64
        }
    }
}
