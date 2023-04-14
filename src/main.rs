use core::num;
use std::fs::File;
use std::io::{Write, BufReader, Read};
use std::iter::successors;
use std::thread;
use std::time::Instant;
use num_bigint::BigUint;

//next step, imrpove to seive ot atkin, and load the prime numbers sequentally instead of storing them in vecs to  see if imoproves

fn main() {
    let thread_count = 7;

    let start = Instant::now();

    let v = &primes_less_than(100_000_000)[0..1_000_000];
    let duration = start.elapsed();
    println!("seive done {duration:?}");

    let start = Instant::now();
    let chunks = v.chunks(v.len()/thread_count+1);

    let chunks = chunks.map(|chunk| chunk.to_vec()).collect::<Vec<_>>();

    let mut handles = vec![];
    for chunk in chunks {
        let h = thread::spawn(move || {
            return multiply_vec(chunk)
        });
        handles.push(h);

    }
    let mut r = BigUint::from(1_u8);
    for handle in handles {
        let result = handle.join().unwrap();
        r*=result;
    }
    //try to multiply this with fold
    let duration = start.elapsed();
    println!("primorial done {duration:?}");

    let start = Instant::now();
    
    let mut file = File::create("new.txt").unwrap();
    file.write_all(r.to_str_radix(10).as_bytes()).unwrap();

    let duration = start.elapsed();
    println!("file done {duration:?}");
    
    let mut file = File::open("new.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let num_chars = contents.chars().count();
    println!("{}",num_chars);

    assert_eq!(num_chars, 6722809)
}

fn multiply_vec(v: Vec<BigUint>) -> BigUint {
    let v = v.into_iter().fold(BigUint::from(1_u8), |acc, x| acc * x);
    BigUint::from(v)
}

fn primes_less_than(n: u128) -> Vec<BigUint> {
    
    if n < 2 {
        return vec![];
    }
    let mut is_prime = vec![true; n as usize];
    
    is_prime[0] = false;
    is_prime[1] = false;
    
    for i in 2..f64::sqrt(n as f64) as u128 {
        if is_prime[i as usize] {
            for x in (i*i..n).step_by(i as usize) {
                is_prime[x as usize] = false
            }
        }
    }

    let mut result: Vec<BigUint> = vec![];
    
    for i in 0..n {
        if is_prime[i as usize] {
            result.push(i.into())
        }
    }
    result
}