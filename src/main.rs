use std::{time::Instant, vec};
use num_bigint::BigUint;

//next step, imrpove to seive ot atkin, and load the prime numbers sequentally instead of storing them in vecs to  see if imoproves

fn main() {
    let thread_count = 9; //7

    let start = Instant::now();

    let v = &sieve_of_atkin(15_485_863);
    let duration = start.elapsed();
    println!("seive done {duration:?}");
    let start = Instant::now();
    let chunk_size = v.len()/thread_count+1;

    let chunks = v.chunks(chunk_size);

    let chunks = chunks.map(|chunk| chunk.to_vec()).collect::<Vec<_>>();

    let mut handles = vec![];
    for chunk in chunks {
        let h = std::thread::spawn(move || {
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
    
    //let contents = r.to_str_radix(10);
    //let num_chars = contents.chars().count();
    //println!("{}",num_chars);

    //assert_eq!(num_chars, 6722809)
}
// do real testing
fn test_seives() {
    let start = Instant::now();
    let _= &primes_less_than(100_000_000);
    let duration = start.elapsed();
    println!("Erasthoatos done {duration:?}");


    let start = Instant::now();
    let _ = &sieve_of_atkin(100_000_000);
    let duration = start.elapsed();
    println!("Atikin done {duration:?}");
}

fn multiply_vec(v: Vec<BigUint>) -> BigUint {
    let v = v.into_iter().fold(BigUint::from(1_u8), |acc, x| acc * x);
    BigUint::from(v)
}
//It makes more sense to use usize here, as the likelyhood that i will be using
//Values that are larger than usize::MAX are low
//using usize is also a good practice due to the way the function is used
//comared to other rust functions.
fn sieve_of_atkin(limit: usize) -> Vec<BigUint> {
    //check the speed of converting list
    let mut sieve = vec![false; limit+1];
    let mut primes: Vec<BigUint> = vec![];

    if limit >= 2 {primes.push(BigUint::from(2_u8))}
    if limit >= 3 {primes.push(BigUint::from(3_u8))}

    //try seeing if you can use iterators to speed this up?!
    for x in 1..=((limit as f64).sqrt() as usize) {
        for y in 1..=((limit as f64).sqrt() as usize) {
            let n = 4 * x * x + y * y;
            if n <= limit && (n % 12 == 1 || n % 12 == 5) {
                sieve[n] ^= true;
            }

            let n = 3 * x * x + y * y;
            if n <= limit && n % 12 == 7 {
                sieve[n] ^= true;
            }

            let n = 3 * x * x - y * y;
            if x > y && n <= limit && n % 12 == 11 {
                sieve[n] ^= true;
            }
        }
    }    
    //remember to eventually do the multiplication here to check for preformance
    for r in 5..=((limit as f64).sqrt() as usize) {
        if sieve[r] {
            let mut i = r * r;
            while i <= limit {
                sieve[i] = false;
                i += r * r;
            }
        }
    }

    // Collect primes
    for (i, is_prime) in sieve.iter().enumerate() {
        if *is_prime {
            primes.push(BigUint::from(i));
        }
    }
    //im going to try and having the result of primes should be Bigint already or not
    primes

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