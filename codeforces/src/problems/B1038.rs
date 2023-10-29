
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

/// n > 2
/// 
/// n even
/// sum = n(n + 1)/2 = n^2/2 + n/2
/// sum(S1) = n^2/2, sum(S2) = n/2
/// gcd = n/2
/// 
/// n odd
/// n = 2k+1
/// sum = (2k+1)^2/2 + (2k+1)/2 = 4k(k+1)/2 + (2k+2)/2 = 2k(k+1) + (k+1)
/// sum(S1) = 2k(k+1), sum(S2) = k+1
/// gcd = k+1
fn main() {
    let mut s = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let n = s.next::<i64>();
    if n < 3 {
        writeln!(out, "No").unwrap();
        return;
    } else if n == 3 {
        writeln!(out, "Yes\n1 2\n2 1 3").unwrap();
        return;
    }
    
    writeln!(out, "Yes").unwrap();
    let ele;
    if n%2 == 0 {
        ele = n/2;
        writeln!(out, "1 {}", n/2).unwrap();
        write!(out, "{} 1 ", n-1).unwrap();
    } else {
        ele = (n-1)/2;
        writeln!(out, "2 1 {}", (n-1)/2).unwrap();
        write!(out, "{} ", n-2).unwrap();
    }

    for i in 2..=n {
        if i != ele {
            write!(out, "{i} ").unwrap();
        }
    }
}