
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

    let t = s.next::<usize>();
    for _ in 0..t {
        let n = s.next::<usize>();
        let a = (0..n).map(|_| s.next::<usize>()).collect::<Vec<_>>();

        let m = s.next::<usize>();
        let b = (0..m).map(|_| s.next::<usize>());

        let ans: usize = a[b.sum::<usize>()%n];
        writeln!(out, "{ans}", ).unwrap();
    }
}