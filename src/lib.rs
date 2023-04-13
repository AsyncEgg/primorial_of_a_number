pub fn primes_less_than(n: u128) -> Vec<u128> {
    
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

    let mut result: Vec<u128> = vec![];
    
    for i in 0..n {
        if is_prime[i as usize] {
            result.push(i)
        }
    }
    result
}
