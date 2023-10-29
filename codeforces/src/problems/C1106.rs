
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

    let n = s.next::<u64>();    
    let mut a = (0..n).into_iter().map(|_| s.next::<u64>()).collect::<Vec<_>>();
    a.sort();
    let mut a = a.into_iter();
    let mut sum = 0;
    for i in 0..(n/2) {
        if i == n/2 {
            sum += (a.next().unwrap()+a.next().unwrap()+a.next().unwrap()).pow(2);
        } else {
            sum += (a.next().unwrap()+a.next_back().unwrap()).pow(2);
        }
    }
    writeln!(out, "{sum}").unwrap();
}