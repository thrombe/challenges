
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

    let mut k = s.next::<u64>();
    let limit = u64::pow(10, 18);
    let mut num = 0;
    while k > 0 {
        if k == 1 {
            num *= 10;
            num += 4; // final number should be > 0
            k -= 1;
        } else {
            num *= 10;
            num += 8;
            k -= 2;
        }
        if num > limit {
            write!(out, "-1").unwrap();
            return;
        }
    }
    write!(out, "{num}").unwrap();
}