
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
    let l = s.next::<u64>();
    let r = s.next::<u64>();

    let mut min = 1 << l; // biggest element should be 1 << l-1
    min -= 1; // -1 cuz summation 2^n === 2^m-1
    min += n-l;

    let mut max = 1 << r;
    max -= 1;
    max += (n-r)*(1 << (r-1));

    writeln!(out, "{min} {max}").unwrap();
}