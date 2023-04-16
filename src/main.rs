use std::{time::Instant, vec};
use num_bigint::BigUint;

//next step, imrpove to seive ot atkin, and load the prime numbers sequentally instead of storing them in vecs to  see if imoproves

fn main() {
    let thread_count = 9; //7

    let start = Instant::now();

    let v = &optimized_sieve_of_eratosthenes(15_485_863);//holy crap i somehow kept this as a usize and converted it later??!??!!?
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
// do real testing of all functions

fn multiply_vec(v: Vec<usize>) -> BigUint {
    let v = v.into_iter().fold(BigUint::from(1_u8), |acc, x| acc * x);
    BigUint::from(v)
}
//It makes more sense to use usize here, as the likelyhood that i will be using
//Values that are larger than usize::MAX are low
//using usize is also a good practice due to the way the function is used
//comared to other rust functions.

//This oprimed version of the seive of eratosthenes is faster, due to the algorithm taking less calculatons
fn optimized_sieve_of_eratosthenes(limit: usize) -> Vec<usize> {
    if limit < 2 {
        return Vec::new();
    }
    //find bettr ways to do things :D
    let mut primes = vec![2];
    let sieve_limit = (limit - 1) / 2;
    let cross_limit = ((((limit as f64).sqrt()) as usize - 1) / 2) as usize; //improve this line
    let mut sieve = vec![false; sieve_limit + 1];

    for i in 1..=cross_limit {
        if !sieve[i] {
            let prime = 2 * i + 1;
            for j in ((i * (prime + 1))..=sieve_limit).step_by(prime) {
                sieve[j] = true;
            }
        }
    }

    for i in 1..=sieve_limit {
        if !sieve[i] {
            primes.push(2 * i + 1);
        }
    }

    primes
}