
use std::time;
fn main() {
    let now = time::Instant::now();
    primes_upto(100);
    println!("{:?}", now.elapsed())
}

fn primes_upto(n: u64) -> Vec<u64> {
    let sze = (n/2) as usize;
    let sqr = (n as f64).sqrt() as usize+1;
    let mut nums = vec![false; sze];
    nums[0] = true; // ignoring 1
    // true is composite and false is potential prime
    let mut primes: Vec<u64> = Vec::new();
    primes.push(2);
    for i in 1..sqr {
        if nums[i] {continue};
        let prime = i*2+1;
        primes.push(prime as u64);
        let mut j = prime*prime/2;
        while j < sze {
            nums[j] = true;
            j += prime;
        }
    }
    for i in sqr..sze {
        if !nums[i] {
            primes.push((i*2+1) as u64);
        }
    }
    println!("{:?}", primes);
    primes
}