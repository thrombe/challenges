
use std::io::{BufWriter, stdin, stdout, Write};
 
#[derive(Default)]
struct Scanner {
    buffer: Vec<String>
}
impl Scanner {
    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut input = String::new();
            stdin().read_line(&mut input).expect("Failed read");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }
}

// only 1 multiplication needed (technically), not necessarily in code
// rest should be sqrt functions
// 2^a * 3^b * 5^c -> 2*3*5 would be the smallest possible
// no of sqrts required would just be -> how many times can i half (if even) (if not even, +=1) the highest power till i get 1
fn main() {
    let mut s = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let n = s.next::<u64>();
    if n == 1 {
        writeln!(out, "1 0").unwrap();
        return;
    }

    let factorisation = prime_factorisation(n);
    let mut smallest_num = 1;
    let mut min_trailing_zeros = u32::MAX;
    let max_power = *factorisation.iter().map(|(prime, power)| {
        smallest_num *= prime;
        min_trailing_zeros = u32::min(min_trailing_zeros, power.trailing_zeros());
        power
    } ).max().unwrap();

    if smallest_num == n { // if prime, cant get smaller
        writeln!(out, "{smallest_num} 0").unwrap();
        return;
    }

    // the next line and the commented code is equivalent for this perpose
    // basically the biggest power of 2 greater than max_power
    let sqrt_count = 64 - max_power.leading_zeros()-1 + if max_power.count_ones() > 1 {1} else {0};
    // let mut sqrt_count = 0;
    // while max_power > 1 {
    //     if max_power % 2 == 1 {
    //         max_power += 1;
    //     }
    //     max_power /= 2;
    //     sqrt_count += 1;
    // }

    // if min_trailing_zeros is less, then while square rooting, somewhere we need to multiply as we cannot divide every power by 2
    let multiplications_needed = sqrt_count > min_trailing_zeros;
    let operations_count = sqrt_count + if multiplications_needed {1} else {0};
    // dbg!(&factorisation, sqrt_count, smallest_num, multiplications_needed);
    writeln!(out, "{smallest_num} {operations_count}").unwrap();
}

// (prime, power)
fn prime_factorisation(mut n: u64) -> Vec<(u64, u64)> {
    let mut factorisation = vec![];
    let mut prime = 3;
    let mut power = 0;

    // taking care of 2 in seperate loop to prevent an if condition in the other loop (2 is only even prime)
    while n%2 == 0 {
        n /= 2;
        power += 1;
    }
    if power != 0 {
        factorisation.push((2, power));
    }

    while n > 1 {
        power = 0;
        while n % prime == 0 {
            n /= prime;
            power += 1;
        }
        if power != 0 {
            factorisation.push((prime, power));
        }
        prime += 2;
    }
    factorisation
}
