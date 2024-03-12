
use num_bigint::{BigUint, ToBigUint};
use num_traits::{Euclid, Pow};
use sha1::{Digest, Sha1};

extern crate tokio;

// const PRIME_ORDER_BIG_UINT: BigUint = BigUint::from_str_radix("512625", 10u32).unwrap();
const PRIME_ORDER: u64 = 512625;
// const GENERATOR:  =

#[tokio::main]
async fn main() {
    let point = hash_to_curve("Hello world!".to_owned());
    println!("{:?} is on the curve", point);
}

#[derive(Debug)]
struct Point {
    pub x: u64,
    pub y: u64,
}

fn hash_to_curve(input: String) -> Result<Option<u64>, ()> {
    let mut counter: u64 = 0;
    loop {
        // Generate hash
        let result =
            Sha1::digest(&[&input[..], &counter.to_string()].concat());

        let midpoint = result.len() / 2;
        let first_half = &result[..midpoint];
        let second_half = &result[midpoint..];

        let mut buf: Vec<u8> = Vec::new();
        buf.extend_from_slice(first_half);
        let x = BigUint::from_bytes_be(&buf);

        buf.clear();
        buf.extend_from_slice(second_half);
        let y = BigUint::from_bytes_be(&buf);
        let x = Euclid::rem_euclid(&x, &PRIME_ORDER.to_biguint().unwrap());
        let y = Euclid::rem_euclid(&y, &PRIME_ORDER.to_biguint().unwrap());

        let point = Point {
            x: u64::try_from(x.clone()).unwrap(),
            y: u64::try_from(y.clone()).unwrap(),
        };

        if is_on_curve(&point) {
            println!("Found on the curve point: {:?}", point);
            return Ok(Some(point.x));
        }
        counter += 1;
        if counter == 2u64.pow(32) {
            return Err(());
        }
    }
}

fn is_on_curve(point: &Point) -> bool {
    let y_squared = point.y.pow(2).rem_euclid(PRIME_ORDER);
    let x_cubed = point.x.pow(3).rem_euclid(PRIME_ORDER);
    y_squared == x_cubed + 7
}
