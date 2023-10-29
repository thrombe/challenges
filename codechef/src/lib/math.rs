/// euclids gcd algorithm
///
/// let g = gcd(a, b)
/// then g divides a and g divides b
/// so g has to divide |a-b| => g divides a%b (let a > b)
fn gcd<T>(mut a: T, mut b: T) -> T
where
    T: std::ops::Rem<Output = T> + std::cmp::PartialOrd + Copy + From<u8>,
{
    if b > a {
        let tmp = a;
        a = b;
        b = tmp;
    }
    if b == 0.into() {
        return a;
    }
    if a == b {
        return a;
    }
    gcd(a % b, b)
}

// returns primes from 2 upto (not including) n
fn primes_upto<T>(n: T) -> Vec<T>
where
    T: Into<usize> + From<usize> + std::ops::Mul + std::ops::Div + Copy,
{
    let sze = n.into() / 2_usize;
    let sqr = (n.into() as f64).sqrt() as usize + 1;
    let mut nums = vec![false; sze];
    nums[0] = true; // ignoring 1
                    // true is composite and false is potential prime
    let mut primes: Vec<T> = Vec::new();
    primes.push(2_usize.into());
    for i in 1..sqr {
        if nums[i] {
            continue;
        };
        let prime: usize = i * 2_usize + 1;
        primes.push(prime.into());
        let mut j = prime * prime / 2;
        while j < sze {
            nums[j] = true;
            j += prime;
        }
    }
    for i in sqr..sze {
        if !nums[i] {
            primes.push((i * 2_usize + 1).into());
        }
    }
    // println!("{:?}", primes);
    primes
}

// (prime, power)
fn prime_factorisation<T>(mut n: T) -> Vec<(T, T)>
where
    T: std::ops::SubAssign
        + std::ops::AddAssign
        + std::ops::DivAssign
        + std::ops::Rem<Output = T>
        + From<u8>
        + std::cmp::PartialEq
        + std::cmp::PartialOrd
        + Copy,
{
    let mut factorisation = vec![];
    let mut prime: T = 3.into();
    let mut power: T = 0.into();

    // taking care of 2 in seperate loop to prevent an if condition in the other loop (2 is only even prime)
    while n % 2.into() == 0.into() {
        n /= 2.into();
        power += 1.into();
    }
    if power != 0.into() {
        factorisation.push((2.into(), power));
    }

    while n > 1.into() {
        power = 0.into();
        while n % prime == 0.into() {
            n /= prime;
            power += 1.into();
        }
        if power != 0.into() {
            factorisation.push((prime, power));
        }
        prime += 2.into();
    }
    factorisation
}

fn prime_factorisation_2<T>(primes: &Vec<T>, mut n: T) -> Vec<(T, T)>
where
    T: std::ops::SubAssign
        + std::ops::AddAssign
        + std::ops::DivAssign
        + std::ops::Rem<Output = T>
        + From<u8>
        + std::cmp::PartialEq
        + std::cmp::PartialOrd
        + Copy,
{
    let mut fac = vec![];
    for &p in primes {
        let mut cnt = 0.into();
        while n % p == 0.into() {
            n /= p;
            cnt += 1.into();
        }
        if cnt > 0.into() {
            fac.push((p, cnt));
        }
        if n == 1.into() {
            break;
        }
    }
    fac
}

fn power_of_2_gte<T>(n: T) -> u32
where
    T: Into<u64>,
{
    let n = n.into();
    // the smallest power of 2 greater than or equal to n
    let power = 64
        + if n.count_ones() > 1 || n < 2 {1} else {0} // this in the middle as if n == 0, it still works
        - n.leading_zeros()-1;
    power
}
