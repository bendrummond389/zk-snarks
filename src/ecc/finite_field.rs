use elliptic_curve::{sec1::ToEncodedPoint, group::prime::PrimeCurveAffine};
use k256::SecretKey;
use k256::{EncodedPoint, Secp256k1, AffinePoint};
extern crate rand;

pub fn generate_keys() {
    let mut rng = rand::thread_rng();
    let secret_key = SecretKey::random(&mut rng);

    let public_key = secret_key.public_key();

    println!("Private Key: {:?}", secret_key);
    println!("Public Key: {:?}", public_key.to_encoded_point(false)); // false for uncompressed form
}

pub fn point_operations() {
    let g = &AffinePoint::generator();
    
    println!("{:?}", g)
}
