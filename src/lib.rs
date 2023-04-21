use num_bigint::BigUint;

pub fn primorial(number_of_primes: usize) -> BigUint {

    let start = std::time::Instant::now();//Timer started

    let primes = &optimized_sieve_of_eratosthenes(number_of_primes*16)[0..number_of_primes];
    
    let duration = start.elapsed(); //Timer Finished
    println!("Duration of prime generation | {duration:?} |");
    
    let start = std::time::Instant::now(); //Timer started
    
    //The following code splits the vector into smaller vectors; the
    //value that controls this thread_count should be adjusted based on
    //the size of the number of primes and the number of cpu cores you have
    let thread_count = 9;
    let chunk_size = primes.len()/thread_count+1; 
    let chunks = primes.chunks(chunk_size);
    //Chunks are converted into vectors
    let chunks = chunks.map(|chunk| chunk.to_vec()).collect::<Vec<_>>();

    let mut handles = vec![];
    for chunk in chunks {
        let h = std::thread::spawn(move || { //Threads created and values multiplied
            return multiply_vec(chunk)
        });
        handles.push(h);

    }
    let mut r = BigUint::from(1_u8);
    for handle in handles { //Threads are handled here
        let result = handle.join().unwrap();
        r*=result;  //Final values multiplied together
    }
    
    let duration = start.elapsed(); //Timer Finished
    println!("Duration of Primorial numbers | {duration:?} |");
    r
}
//Fastest possible multiplication meathod
fn multiply_vec(v: Vec<usize>) -> BigUint {
    let v = v.into_iter().fold(BigUint::from(1_u8), |acc, x| acc * x);
    BigUint::from(v)
}

//Function inspired from https://www.geeksforgeeks.org/binary-search/

//Function uses the Babylonian meathod for finding squre roots which is
//faster than rust's built in sqrt() function.
fn sqrt_binary_search(n: usize) -> usize { 
    let mut low = 0;
    let mut high = n;

    while low <= high {
        let mid = (low + high) / 2;
        let square = mid * mid;

        if square == n {
            return mid;
        } else if square < n {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    low - 1
}

//This oprimed version of the seive of eratosthenes is
//faster, due to the algorithm taking less calculatons.

//It makes more sense to use usize here, as the likelyhood that i will be using
//values that are larger than usize::MAX are low; but if needed the
//function can be tweaked to use BigUints which is limited by
//the ammount of ram your computer has.

//Using usize is also a good practice due to the way the function is used
//comared to other rust functions.
pub fn optimized_sieve_of_eratosthenes(number_of_primes: usize) -> Vec<usize> {
    if number_of_primes == 0 {
        return vec![1];//return vec if limit is 0
    }
    if number_of_primes == 1 {
        return vec![2];//return vec if limit is 1
    }
    //:D
    let mut primes = vec![2];
    let sieve_limit = (number_of_primes - 1) / 2;
    let cross_limit = (sqrt_binary_search(number_of_primes) - 1) / 2; //improve this line
    let mut sieve = vec![false; sieve_limit + 1];

    for i in 1..=cross_limit {
        if !sieve[i] {
            let prime = 2 * i + 1;
            for j in ((i * (prime + 1))..=sieve_limit).step_by(prime) {
                sieve[j] = true;
            }
        }
    }
    //check primes
    for i in 1..=sieve_limit {
        if !sieve[i] {
            primes.push(2 * i + 1);
        }
    }

    primes
}