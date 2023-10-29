
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
    let m = s.next::<u64>();
    s.next::<u64>();
    let front = (0..m).map(|_| s.next::<u64>()).collect::<Vec<u64>>();
    let left = (0..n).map(|_| s.next::<u64>()).collect::<Vec<u64>>();
    let top = (0..n).map(|_| (0..m).map(|_| s.next::<u64>()).collect::<Vec<u64>>()).collect::<Vec<Vec<u64>>>();
    
    (0..n as usize).for_each(|i| (0..m as usize).for_each(|j| {
        let k = if top[i][j] == 0 {
            0
        } else {
            u64::min(front[j], left[i])
        };
        if j as u64 == m-1 {
            write!(out, "{k}\n").unwrap();
        } else {
            write!(out, "{k} ").unwrap();
        }
    } ));
}