
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

    let mut n = s.next::<u32>();
    let k = s.next::<u32>();
    let mut del = 0;
    let seq = s.next::<String>().chars().filter(|&c| -> bool {
        if n > k && c == '(' {
            n -= 2;
            del += 1;
            false
        } else if del > 0 && c == ')' {
            del -= 1;
            false
        } else {
            true
        }
    }).collect::<String>();

    writeln!(out, "{seq}").unwrap();
}