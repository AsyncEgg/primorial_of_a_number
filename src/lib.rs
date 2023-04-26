use std::{time::Duration, fs::File, io::{BufWriter, Write, BufReader, Read}, str::FromStr, thread};

use num_bigint::BigUint;

//The primorial of a number is the multiplication of n primes.
//first 6 prime numbers
//E.g. 6 -> 2×3×5×7×11×13 -> 30030
pub fn primorial(number_of_primes: usize) -> (BigUint, Duration, Duration) {

    let prime_start = std::time::Instant::now();//Timer started

    let primes = &optimized_sieve_of_eratosthenes(number_of_primes*16)[0..number_of_primes];
    
    let prime_duration = prime_start.elapsed(); //Timer Finished
    
    let primorial_start = std::time::Instant::now(); //Timer started
    
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
        let handle = std::thread::spawn(move || { //Threads created and values multiplied
            multiply_vec(chunk)
        });
        handles.push(handle);

    }
    let mut r = BigUint::from(1_u8);
    for handle in handles { //Threads are handled here
        let result = handle.join().unwrap();
        r*=result;  //Final values multiplied together
    }
    
    let primorial_duration = primorial_start.elapsed(); //Timer Finished
    (r, prime_duration, primorial_duration)
}
//Fastest possible multiplication meathod
fn multiply_vec(v: Vec<usize>) -> BigUint {
    v.into_iter().fold(BigUint::from(1_u8), |acc, x| acc * x)
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

#[derive(Copy, Clone, Debug)]
pub enum WriteMode {
    Bin,
    Txt,
    None
}

pub fn write_biguint_to_file(biguint: num_bigint::BigUint, write_mode: &WriteMode) -> Result<String, std::io::Error> {
    match write_mode {
        WriteMode::Bin => {
            let start = std::time::Instant::now();
            let bytes = biguint.to_bytes_le();

            let thread_handle = thread::spawn(move || {
                
                let file = File::create("biguint_data.bin")?;
                let mut writer = BufWriter::new(file);
                writer.write_all(&bytes)?;
                writer.flush()?;
                
                Ok::<(), std::io::Error>(())
            });

            thread_handle.join().unwrap()?;
            let duration = start.elapsed();
            Ok(format!("{:?}",duration))
        }
        WriteMode::Txt => {
            let start = std::time::Instant::now();
            let string = biguint.to_str_radix(10);

            let thread_handle = thread::spawn(move || {
                
                let file = File::create("biguint_base10.txt")?;
                let mut buffered_writer = BufWriter::new(file);
                write!(buffered_writer, "{}", string)?;
                Ok::<(), std::io::Error>(())
            });

            thread_handle.join().unwrap()?;
            let duration = start.elapsed();
            Ok(format!("{:?}",duration))
        }
        WriteMode::None => Ok(String::from("No file")),
    }
}

pub enum ReadMode {
    Bin,
    Txt,
    None
}

pub fn read_file_to_biguint(read_mode: &ReadMode) -> std::io::Result<(String, String, BigUint)> {
    
    match read_mode {
        &ReadMode::Bin => {
            let start = std::time::Instant::now();//Timer started


            let mut file = File::open("biguint_data.bin")?;
            let mut bytes = Vec::new();
            file.read_to_end(&mut bytes)?;
            let biguint = BigUint::from_bytes_le(&bytes);
            
            let duration = start.elapsed();
            Ok((format!("{:?}",duration), biguint.to_str_radix(10), biguint))//Elapsed time return
        }
        &ReadMode::Txt => {
            let start = std::time::Instant::now();//Timer started

            let file = File::open("biguint_base10.txt")?;
            let mut buf_reader = BufReader::new(file);
            let mut contents = String::new();
            buf_reader.read_to_string(&mut contents)?;

            let duration = start.elapsed();
            Ok((format!("{:?}",duration), contents.clone(), BigUint::from_str(&contents).unwrap()))//Elapsed time return
        }
        &ReadMode::None => {
            Ok((String::from("No file"), String::new(), BigUint::new(vec![])))
        }
    }
}