use std::str::FromStr;

use is_odd::IsOdd;
use num_bigint::{BigUint, ToBigUint};
use num_traits::{Euclid, Num};
use sha1::{Digest, Sha1};

// const PRIME_ORDER_BIG_UINT: BigUint = BigUint::from_str_radix("512625", 10u32).unwrap();
const PRIME_ORDER: u64= 512625;
// const GENERATOR:  =
fn main() {
    let point = hash_to_curve("Hello world!");
    println!("{:?}", point);
}

#[derive(Debug)]
struct Point {
    pub x: u64,
    pub y: u64,
}

fn hash_to_curve(x: &str) -> Result<u64, ()> {
    // Generate hash
    let result = Sha1::digest(x);

    let midpoint = result.len() / 2;
    let first_half = &result[..midpoint];
    let second_half = &result[midpoint..];
    println!("Sliced it up");

    let mut buf: Vec<u8> = Vec::new();
    buf.extend_from_slice(first_half);
    let x = BigUint::from_bytes_be(&buf);

    buf.clear();
    buf.extend_from_slice(second_half);
    let y = BigUint::from_bytes_be(&buf);
    let x = Euclid::rem_euclid(&x, &PRIME_ORDER.to_biguint().unwrap());
    let y = Euclid::rem_euclid(&y, &PRIME_ORDER.to_biguint().unwrap());

    let point = Point {
        x: u64::try_from(x.clone()).map_err(|_| ())?,
        y: u64::try_from(y.clone()).map_err(|_| ())?,
    };

    if is_on_curve(&point) {
        let formatted_point = format!(
            "{}{}",
            if IsOdd::is_odd(&point.y) { "02" } else { "03" },
            point.x
        );
        let value = u64::from_str_radix(&formatted_point, 16).unwrap();
        Ok(value)
    } else {
        let x = (x + BigUint::new(vec![1])).to_string();
        hash_to_curve(&x)
    }
}

fn is_on_curve(point: &Point) -> bool {
    let y_squared = point.y.pow(2).rem_euclid(PRIME_ORDER);
    let x_cubed = point.x.pow(3).rem_euclid(PRIME_ORDER);
    y_squared == x_cubed + 7
}

