// kata level 4

fn main() {
    println!("{:?}", sum_of_divided(vec![12, 15]));
}

fn primes_under(n: u64) -> Vec<u64> {
    // let n = (n as f64).sqrt() as usize + 1;
    let n = n as usize;
    if n < 2 {
        return vec![];
    }
    let mut prime_sieve = vec![false ; n + 1];
    let mut primes = vec![];
    prime_sieve[0] = true; // 0
    prime_sieve[1] = true; // 1
    for pp in 2..n {
        if prime_sieve[pp] {
            continue;
        }
        let mut i = 2*pp;
        while i < n+1 {
            prime_sieve[i] = true;
            i += pp;
        }
        primes.push(pp as u64);
    }
    primes
}

fn sum_of_divided(l: Vec<i64>) -> Vec<(i64, i64)> {
    let m = l.iter()
                // .max_by(|&x, &y| x.abs().cmp(&y.abs()))
                .map(|&x| x.abs()).max()
                .unwrap_or_else(|| 0);
    let mut res = vec![];
    for p in primes_under(m as u64 + 1) {
        let p = p as i64;
        let mut sum = 0;
        let mut include_this = false;
        for &num in &l {
            if num % p == 0 {
                include_this = true;
                sum += num;
            }
        }
        if !include_this {continue}
        res.push((p, sum));
    }
    res
}