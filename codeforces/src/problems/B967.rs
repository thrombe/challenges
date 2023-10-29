
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
    let a = s.next::<u64>();
    let b = s.next::<u64>();
    let s1 = s.next::<u64>();

    let mut si = (0..n-1).into_iter().map(|_| s.next::<u64>()).collect::<Vec<u64>>();
    si.sort();
    let mut s_sum = si.iter().sum::<u64>() + s1;

    let mut block_count = 0;
    loop {
        let w1 = s1*a/s_sum;
        if w1 >= b {break}
        s_sum -= si.pop().unwrap();
        block_count += 1;
    }

    writeln!(out, "{block_count}").unwrap();
}