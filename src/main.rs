use std::{fs::{File, remove_file}, io::{Read, Write}, path::Path};

use num_bigint::BigUint;
use primorial_of_a_number::ThreadPool;

fn main() {
    if !Path::new("primes.txt").exists() {
        let mut file = File::create("primes.txt").expect("Help");
        for n in primes_less_than(100_000) {
            file.write_all(&n.to_be_bytes()).expect("Help")
        }    
    }

    let primes = get_primes();
    let mut ids = vec![];
    
    let pool = ThreadPool::new(5);

    for (index, x) in (0..1000).enumerate() {
        let primes = primes.clone();
        pool.execute(move || {
            let primordial = primordial(x, primes);
            println!("{index}: {primordial}")   
        });
        ids.push(index);
    }

    ids.sort();
    
    for (count, id) in ids.iter().enumerate() {
        assert_eq!(count, *id);
    }

    println!("Assertation completed; all threads ran correctly; executing then cleaning up");
    remove_file("primes.txt").unwrap();
    
}

fn primordial(n: u128, primes: Vec<u128>) -> BigUint {
    let mut result = BigUint::from(1_u8);

    for x in 0..n {
        result = BigUint::from(result) * BigUint::from(primes[x as usize]);
    }

    result
}

fn get_primes() -> Vec<u128> {
    let mut file = File::open("primes.txt").expect("help");

    let mut buffer = [0u8; 16];
    let mut primes = Vec::new();

    while let Ok(n) = file.read(&mut buffer) {
        if n == 0 {
            break;
        }
        primes.push(u128::from_be_bytes(buffer));
    }

    primes
}

fn primes_less_than(n: u128) -> Vec<u128> {
    
    if n < 2 {
        return vec![];
    }
    let mut is_prime = vec![true; n as usize];
    
    is_prime[0] = false;
    is_prime[1] = false;

    for i in 2..isqrt(n) {
        if is_prime[i as usize] {
            for x in (i*i..n).step_by(i as usize) {
                is_prime[x as usize] = false
            }
        }
    }

    let mut result = vec![];
    
    for i in 0..n {
        if is_prime[i as usize] {
            result.push(i)
        }
    }

    result
}

fn isqrt(n: u128) -> u128 {
    f64::sqrt(n as f64) as u128
}