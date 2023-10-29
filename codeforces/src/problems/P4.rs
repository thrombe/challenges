
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

    let q = s.next::<usize>();
    for _ in 0..q {
        let s1 = s.next::<String>();
        let s2 = s.next::<String>();
    
        if a_div_b(&s1, &s2) {
            for _ in 0..(s1.len()*s2.len()/(gcd(s1.len() as u64, s2.len() as u64) as usize)) {
                write!(out, "{}", if s1.len() < s2.len() {&s1} else {&s2}).unwrap();
            }
        } else {
            write!(out, "-1").unwrap();
        }
        writeln!(out, "").unwrap();
    }
}

fn a_div_b(a: &str, b: &str) -> bool {
    let s1;
    let s2;
    if b.len() < a.len() {
        let t = a;
        s1 = b;
        s2 = t;
    } else {
        s1 = a;
        s2 = b;
    }
    let s1 = s1.chars().collect::<Vec<char>>();
    let s2 = s2.chars().collect::<Vec<char>>();

    for i in 0..s2.len() {
        if s2[i] != s1[i%s1.len()] {
            return false
        }
    }

    true
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

