
use num_bigint::{BigUint, ToBigUint};
use num_traits::Euclid;
use sha1::{Digest, Sha1};

extern crate tokio;

// const PRIME_ORDER_BIG_UINT: BigUint = BigUint::from_str_radix("512625", 10u32).unwrap();
const PRIME_ORDER: u64 = 512625;
// Point { x: 83388, y: 406073 };
// We hashed to curve to find this.
// Ok(Some(83388)) is on the curve for Hello world!
const GENERATOR: u64 = 83388;
// We have to curve "private key" to get Alice's key
const ALICE_PRIVATE_KEY: u64 = 391164;

#[tokio::main]
async fn main() {
    let mint_private_key_k = hash_to_curve("Bob's Mint".to_owned()).unwrap().unwrap();
    println!("Bob's mint has a private key `k`: {}", mint_private_key_k);
    println!("He generates his public key `K` by multiplying private key {} and GENERATOR {} mod {}", mint_private_key_k, GENERATOR, PRIME_ORDER);
    println!("K = kG");
    let mint_pub_key = (mint_private_key_k * GENERATOR).rem_euclid(PRIME_ORDER);
    println!("We get a public key `K`: {}", mint_pub_key);
    println!("(For Cashu, each amount of ecash is given a public key)");
    println!("Now our client, Alice, sets up `Y` by hashing a secret `x` ('alice') to the curve.");
    let client_secret_x = "alice";
    let client_y = hash_to_curve(client_secret_x.clone().to_owned()).unwrap().unwrap();
    println!("Y = hash_to_curve(x)");
    println!("We get {}", client_y);
    println!("Now Alice can create a blinding factor by using her private key `r`");
    println!("B_ = Y + rG");
    let b_ = ((ALICE_PRIVATE_KEY * GENERATOR).rem_euclid(PRIME_ORDER) + client_y).rem_euclid(PRIME_ORDER);
    println!("We end up with {}, which is passed from Alice (client) to Bob (mint)", b_);
    println!("Bob (mint) then signs this using his private key");
    println!("C_ = kB_");
    let c_ = (mint_private_key_k * b_).rem_euclid(PRIME_ORDER);
    println!("Alice can then unblind things using:");
    println!("C_ - rK = kY + krG - krG = kY = C");
    let rk = (ALICE_PRIVATE_KEY * mint_pub_key).rem_euclid(PRIME_ORDER);
    println!("First: C_ - rK = {}", (c_ - rk).rem_euclid(PRIME_ORDER));
    let ky = (mint_private_key_k * client_y).rem_euclid(PRIME_ORDER);
    let krg = ((mint_private_key_k * ALICE_PRIVATE_KEY).rem_euclid(PRIME_ORDER) * GENERATOR).rem_euclid(PRIME_ORDER);
    println!("Which is also: kY + krG = {}", ((ky + krg).rem_euclid(PRIME_ORDER) - krg).rem_euclid(PRIME_ORDER));
    println!("Which is also kY = {}", ky);
    let c = (c_ - rk).rem_euclid(PRIME_ORDER);
    println!("Which is the unblinded value of C: {}", c);
    // Carol part
    println!("Now Alice pays Carol by providing (x, C) or ({}, {}) in this case.", client_secret_x, c);
    println!("Carol sends (x, C) to Bob (who doesn't know where x comes from) who can then verify.");
    println!("k*hash_to_curve(x) == C");
    let k_hash_to_curve_x = (mint_private_key_k*hash_to_curve(client_secret_x.to_owned()).unwrap().unwrap()).rem_euclid(PRIME_ORDER);
    println!("Does {} equal {}? {}", k_hash_to_curve_x, c, k_hash_to_curve_x == c);
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
