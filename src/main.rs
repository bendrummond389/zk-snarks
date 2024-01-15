#[allow(dead_code)]
mod circuits;
mod ecc;
mod r1cs;
mod utils;
use elliptic_curve::Field;
use k256::Scalar;

use circuits::Circuit;
use ecc::finite_field::point_operations;
use r1cs::r1cs::R1CS;
use std::{collections::HashMap, env};
use utils::polynomial::polynomial::Polynomial;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    // let points = vec![
    //     (Scalar::from(1 as u32), Scalar::from(2 as u32)),
    //     (Scalar::from(3 as u32), Scalar::from(4 as u32)),
    //     (Scalar::from(5 as u32), Scalar::from(6 as u32)),
    // ];

    // // Interpolate and print the result
    // let interpolated_poly = Polynomial::interpolate(&points);


    let a = Scalar::from(70000 as u32);

    let bytes = a.to_bytes();

    println!("{:?}", bytes);

    


}
