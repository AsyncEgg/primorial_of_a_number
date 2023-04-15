use std::time::Instant;
use num_bigint::BigUint;

//next step, imrpove to seive ot atkin, and load the prime numbers sequentally instead of storing them in vecs to  see if imoproves

fn main() {
    let thread_count = 9; //7

    let start = Instant::now();

    let v = &primes_less_than(15_485_867)[0..1_000_000];
    let chunk_size = v.len()/thread_count+1;
    println!("{}",chunk_size);
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