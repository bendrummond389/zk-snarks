use elliptic_curve::Field;
use k256::Scalar;
use std::ops::{Add, Mul};

#[derive(Debug, Clone)]
pub struct Polynomial {
    coefficients: Vec<Scalar>,
}

impl Polynomial {
    pub fn new(coefficients: Vec<Scalar>) -> Self {
        Polynomial { coefficients }
    }

    pub fn zero(degree: usize) -> Self {
        let coefficients = vec![Scalar::ZERO; degree + 1];
        Polynomial { coefficients }
    }

    pub fn interpolate(points: &[(Scalar, Scalar)]) -> Self {
        let mut terms = Vec::with_capacity(points.len());
        for i in 0..points.len() {
            let xi = points[i].0;
            let mut denominator = Scalar::ONE;

            for j in 0..points.len() {
                if i != j {
                    let xj = points[j].0;
                    denominator *= xi - xj;
                }
            }

            terms.push((xi, denominator));
        }

        for (xi, denominator) in terms {
            println!("xi: {:?}, denominator: {:?}", xi, denominator);
        }

        Polynomial {
            coefficients: vec![Scalar::ONE],
        }
        // unimplemented!()
    }
}

impl Add for Polynomial {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        // Pad the shorter polynomial with zeros to match the lengths
        let (mut shorter, mut longer) = if self.coefficients.len() < other.coefficients.len() {
            (self.coefficients.clone(), other.coefficients)
        } else {
            (other.coefficients.clone(), self.coefficients)
        };

        // Add coefficients element-wise
        let mut result_coefficients = Vec::with_capacity(longer.len());
        for (a, b) in shorter.iter().zip(longer.iter()) {
            result_coefficients.push(*a + *b);
        }
        result_coefficients.extend_from_slice(&longer[shorter.len()..]);

        Polynomial {
            coefficients: result_coefficients,
        }
    }
}

impl Mul for Polynomial {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mut result_coefficients =
            vec![Scalar::ZERO; self.coefficients.len() + other.coefficients.len() - 1];

        for (i, &a) in self.coefficients.iter().enumerate() {
            for (j, &b) in other.coefficients.iter().enumerate() {
                result_coefficients[i + j] += a * b;
            }
        }

        Polynomial {
            coefficients: result_coefficients,
        }
    }
}

