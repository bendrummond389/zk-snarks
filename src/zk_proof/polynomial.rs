pub struct Polynomial {
    pub coefficients: Vec<i32>,
}

impl Polynomial {
    pub fn new(coefficients: Vec<i32>) -> Self {
        Polynomial { coefficients }
    }

    pub fn evaluate(&self, x: i32) -> i32 {
        self.coefficients
            .iter()
            .enumerate()
            .fold(0, |acc, (i, &coeff)| acc + coeff * x.pow(i as u32))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polynomial_creation() {
        let coefficients = vec![1, 2, 3];
        let poly = Polynomial::new(coefficients.clone());
        assert_eq!(poly.coefficients, coefficients);
    }

    #[test]
    fn test_polynomial_evaluation() {
        let poly = Polynomial::new(vec![1, 2, 3]);
        assert_eq!(poly.evaluate(0), 1);
        assert_eq!(poly.evaluate(1), 6);
        assert_eq!(poly.evaluate(2), 17);
    }
}
