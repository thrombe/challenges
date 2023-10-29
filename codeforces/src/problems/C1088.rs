
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

    let n = s.next::<usize>();
    let mut a = (0..n).map(|_| s.next::<usize>()).collect::<Vec<_>>();

    writeln!(out, "{}", n+1).unwrap();
    for i in (0..n).rev() {
        let ai = a[i];
        let next_multilple_of_2k = (ai/n +1)*n;
        let diff = next_multilple_of_2k - ai + i;
        for j in 0..=i {
            // map a[i] to k*n + i where k is any whole number
            a[j] += diff;
        }
        writeln!(out, "1 {} {}", i+1, diff).unwrap();
    }
    writeln!(out, "2 {n} {n}").unwrap();
}
