extern crate num_bigint; // 0.4.3
use num_bigint::BigUint;

fn main() {
    let primes = primes_less_than(1_000_000);

    println!("{}",primordial(1_000, &primes));

}

fn primordial(n: u128, primes: &Vec<u128>) -> BigUint {
    let mut result = BigUint::from(1_u8);
    for prime in &primes[..n as usize] {
        result *= BigUint::from(*prime)
    }
    println!("done! {n}");
    result
}

fn primes_less_than(n: u128) -> Vec<u128> {
    
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

    let mut result = vec![];
    
    for i in 0..n {
        if is_prime[i as usize] {
            result.push(i)
        }
    }

    result
}