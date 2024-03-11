use is_odd::IsOdd;
use num_bigint::{BigUint, ToBigUint};
use num_traits::Euclid;
use sha1::{Digest, Sha1};

extern crate tokio;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::task;

// const PRIME_ORDER_BIG_UINT: BigUint = BigUint::from_str_radix("512625", 10u32).unwrap();
const PRIME_ORDER: u64 = 512625;
// const GENERATOR:  =

#[tokio::main]
async fn main() {
    let point = hash_to_curve("Hello world!".to_owned()).await;
    println!("{:?} is on the curve", point);
}

#[derive(Debug)]
struct Point {
    pub x: u64,
    pub y: u64,
}

async fn hash_to_curve(mut input: String) -> Result<Option<u64>, ()> {
    let (tx, mut rx) = mpsc::channel(1000);

    let primes = Arc::new((0..PRIME_ORDER as usize).collect::<Vec<usize>>());
    let primes_ref = Arc::clone(&primes);

    task::spawn(async move {
        for prime in primes_ref.iter() {
            let mut value: Option<u64> = None;
            let mut counter: u64 = 0;
            let mut input = format!("{}.{}", prime, "Hello world!");
            loop {
                // Generate hash
                let result = Sha1::digest(input);

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
                    let formatted_point = format!(
                        "{}{}",
                        if IsOdd::is_odd(&point.y) { "02" } else { "03" },
                        point.x
                    );
                    value = Some(u64::from_str_radix(&formatted_point, 16).unwrap());
                    break;
                }
                input = (x + BigUint::new(vec![1])).to_string();
                counter += 1;
                if counter == PRIME_ORDER {
                    break;
                }
            }
            tx.send(value).await.expect("Failed to send message");
        }
    });

    while let Some(message) = rx.recv().await {
        println!("GOT = {:?}", message);
        return Ok(message);
    }
    Ok(None)
}

fn is_on_curve(point: &Point) -> bool {
    let y_squared = point.y.pow(2).rem_euclid(PRIME_ORDER);
    let x_cubed = point.x.pow(3).rem_euclid(PRIME_ORDER);
    y_squared == x_cubed + 7
}
