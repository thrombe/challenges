
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


fn main() {
    let mut s = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let a = s.next::<u64>();
    let b = s.next::<u64>();
    let x = s.next::<u64>();
    let y = s.next::<u64>();

    let g = gcd(x, y);

    let sx = x/g;
    let sy = y/g;

    writeln!(out, "{}", u64::min(a/sx, b/sy)).unwrap();
}


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