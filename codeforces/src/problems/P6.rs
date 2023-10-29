
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

    let name = s.next::<String>();

    let len = name.len();
    let mut leftover = len%20;
    let rows = len/20 + 1;
    let mut row_len = 20;
    if rows == 1 {
        writeln!(out, "1 {len}").unwrap();
        writeln!(out, "{name}").unwrap();
        return;
    }

    let mut lines = vec![];
    while leftover > 1 {
        if leftover - (rows-1) > 1 {
            leftover -= 1;
            row_len -= 1;
        } else {
            let i = 0;
            for _ in 0..rows {
                lines.push(name[i..(i+row_len)].to_owned());
                row_len -= 1;
            }
        }
    }
}