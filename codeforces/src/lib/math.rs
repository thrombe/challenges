
/// euclids gcd algorithm
///
/// let g = gcd(a, b)
/// then g divides a and g divides b
/// so g has to divide |a-b| => g divides a%b (let a > b)
fn gcd<T>(mut a: T, mut b: T) -> T 
where T: std::ops::Rem<Output=T> + std::cmp::PartialOrd + Copy + From<u64>,
{
    if b > a {
        let tmp = a;
        a = b;
        b = tmp;
    }
    if b == 0_u64.into() {
        return a;
    }
    if a == b {
        return a;
    }
    gcd(a%b, b)
}

// returns primes from 2 upto (not including) n
fn primes_upto<T>(n: T) -> Vec<T> 
where T: Into<usize> + From<usize> + std::ops::Mul + std::ops::Div + Copy
{
    let sze = n.into()/2_usize;
    let sqr = (n.into() as f64).sqrt() as usize+1;
    let mut nums = vec![false; sze];
    nums[0] = true; // ignoring 1
    // true is composite and false is potential prime
    let mut primes: Vec<T> = Vec::new();
    primes.push(2_usize.into());
    for i in 1..sqr {
        if nums[i] {continue};
        let prime: usize = i*2_usize+1;
        primes.push(prime.into());
        let mut j = prime*prime/2;
        while j < sze {
            nums[j] = true;
            j += prime;
        }
    }
    for i in sqr..sze {
        if !nums[i] {
            primes.push((i*2_usize+1).into());
        }
    }
    // println!("{:?}", primes);
    primes
}

// returns an array where arr[i] returns the greatest prime factor of i
// this can be used to quickly do prime factorisation
fn factorisation_sieve<T>(n: T) -> Vec<usize>
    where T: std::ops::SubAssign + std::ops::AddAssign + std::ops::DivAssign + std::ops::Rem<Output=T> + From<usize> + Into<usize> + std::cmp::PartialEq + std::cmp::PartialOrd + Copy
{
    let mut sieve = vec![0;n.into()+1];
    sieve[0] = 1;
    sieve[1] = 1;
    for i in 2..=n.into() {
        if sieve[i] == 0 {
            let mut j = i;
            while j <= n.into() {
                // if sieve[j] == 0 { // this check will give the smallest factor in sieve
                    sieve[j] = i;
                // }
                j += i;
            }
        }
    }
    sieve
}

// returns an array where arr[i] returns the smallest prime factor of i
// this can be used to quickly do prime factorisation
fn factorisation_sieve_smallest<T>(n: T) -> Vec<usize>
    where T: std::ops::SubAssign + std::ops::AddAssign + std::ops::DivAssign + std::ops::Rem<Output=T> + From<usize> + Into<usize> + std::cmp::PartialEq + std::cmp::PartialOrd + Copy
{
    let mut sieve = vec![0;n.into()+1];
    sieve[0] = 1;
    sieve[1] = 1;
    for i in 2..=n.into() {
        if sieve[i] == 0 {
            let mut j = i;
            while j <= n.into() {
                if sieve[j] == 0 { // this check will give the smallest factor in sieve
                    sieve[j] = i;
                }
                j += i;
            }
        }
    }
    sieve
}

// (prime, power)
fn prime_factorisation(n: usize) -> std::collections::HashMap<usize, usize> {
    let sieve = factorisation_sieve(n);
    prime_factorisation_with_factorisation_sieve(n, sieve)
}

// (prime, power)
fn prime_factorisation_with_factorisation_sieve(mut n: usize, sieve: Vec<usize>) -> std::collections::HashMap<usize, usize> {
    let mut factorisation = std::collections::HashMap::new();
    
    while n > 1 {
        let factor = sieve[n];
        if let Some(m) = factorisation.get_mut(&factor) {
            *m += 1;
        } else {
            factorisation.insert(factor, 1);
        }
        n /= factor;
    }
    factorisation
}

fn power_of_2_gte<T>(n: T) -> u32 
where T: Into<u64>
{
    let n = n.into();
    // the smallest power of 2 greater than or equal to n
    
    64 
        + if n.count_ones() > 1 || n < 2 {1} else {0} // this in the middle as if n == 0, it still works
        - n.leading_zeros()-1
}
