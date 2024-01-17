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

    pub fn one(degree: usize) -> Self {
        let coefficients = vec![Scalar::ONE; degree + 1];
        Polynomial { coefficients }
    }

    pub fn evaluate_at(&self, x: Scalar) -> Scalar {
        let mut result = Scalar::ZERO;
        let mut power_of_x = Scalar::ONE;

        for &coeff in &self.coefficients {
            result += coeff * power_of_x;
            power_of_x *= x;
        }

        result
    }

    pub fn basis_polynomial(points: &[(Scalar, Scalar)], i: usize) -> Self {
        let (xi, _) = points[i];

        let mut li = Polynomial::one(0);

        for (j, &(xj, _)) in points.iter().enumerate() {
            if i != j {
                let term = Polynomial::new(vec![-xj, Scalar::ONE]);
                let inverted_denom = (xi - xj).invert().unwrap_or(Scalar::ZERO);
                li = li * (term * inverted_denom)
            }
        }

        li
    }

    pub fn interpolate(points: &[(Scalar, Scalar)]) -> Self {
        let mut result = Polynomial::zero(points.len() - 1);

        for (i, &(_, yi)) in points.iter().enumerate() {
            let basis_poly = Polynomial::basis_polynomial(points, i);

            let term = basis_poly * yi;
            result = result + term;
        }

        result
    }
}

impl Add for Polynomial {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let (mut shorter, mut longer) = if self.coefficients.len() < other.coefficients.len() {
            (self.coefficients.clone(), other.coefficients)
        } else {
            (other.coefficients.clone(), self.coefficients)
        };

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

impl Mul<Scalar> for Polynomial {
    type Output = Self;

    fn mul(self, scalar: Scalar) -> Self::Output {
        let mut result_coefficients = Vec::with_capacity(self.coefficients.len());

        for coeff in self.coefficients.into_iter() {
            result_coefficients.push(coeff * scalar);
        }

        Polynomial {
            coefficients: result_coefficients,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basis_polynomials() {
        let points = vec![
            (Scalar::from(0u64), Scalar::from(0u64)),
            (Scalar::from(1u64), Scalar::from(1u64)),
            (Scalar::from(3u64), Scalar::from(6u64)),
        ];
        for (i, &(xi, _)) in points.iter().enumerate() {
            let basis_poly = Polynomial::basis_polynomial(&points, i);

            assert_eq!(basis_poly.evaluate_at(xi), Scalar::ONE);

            for (j, &(xj, _)) in points.iter().enumerate() {
                if j != i {
                    assert_eq!(basis_poly.evaluate_at(xj), Scalar::ZERO);
                }
            }
        }
    }


    #[test]
    fn test_lagrange_polynomial() {
        let points = vec![
            (Scalar::from(1u64), Scalar::from(3u64)),
            (Scalar::from(2u64), Scalar::from(7u64)),
            (Scalar::from(3u64), Scalar::from(4u64)),
        ];

        let poly = Polynomial::interpolate(&points);

        assert_eq!(poly.evaluate_at(Scalar::from(1u64)), Scalar::from(3u64));
        assert_eq!(poly.evaluate_at(Scalar::from(2u64)), Scalar::from(7u64));
        assert_eq!(poly.evaluate_at(Scalar::from(3u64)), Scalar::from(4u64));
    }
}
