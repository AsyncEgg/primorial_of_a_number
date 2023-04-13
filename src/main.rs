use std::fs::File;
use std::io::Write;
use std::thread;
use std::time::Instant;
use num_bigint::BigUint;
use primorial_of_a_number::primes_less_than;

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
            let mut result = BigUint::from(1_u8);
            for prime in chunk {
                result *= BigUint::from(prime);
            }
            result
        });
        handles.push(h);

    }
    let mut r = BigUint::from(1_u8);
    for handle in handles {
        let result = handle.join().unwrap();
        r*=result;
    }
    let duration = start.elapsed();
    println!("primorial done {duration:?}");


    let start = Instant::now();
    
    let mut file = File::create("n.txt").unwrap();
    file.write_all(r.to_str_radix(10).as_bytes()).unwrap();

    let duration = start.elapsed();
    println!("file done {duration:?}");

}