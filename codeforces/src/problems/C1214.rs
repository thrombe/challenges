
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

    let n = s.next::<u32>();
    if n % 2 == 1 {
        writeln!(out, "No").unwrap();
        return;
    }

    let mut k = 0;
    let mut nve_max = 0;
    s.next::<String>().chars().for_each(|c| {
        if c == '(' {
            k += 1;
        } else if c == ')' {
            k -= 1;
        }
        if k < 0 {
            nve_max = i32::max(nve_max, -k);
        }
    });
    if k == 0 && (nve_max == 1 || nve_max == 0) {
        writeln!(out, "Yes").unwrap();
    } else {
        writeln!(out, "No").unwrap();
    }
}


