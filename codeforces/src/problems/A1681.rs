
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
        let n_max = (0..n).map(|_| s.next::<u32>()).max();
        let m = s.next::<usize>();
        let m_max = (0..m).map(|_| s.next::<u32>()).max();
        
        // if alice first
        if n_max >= m_max {
            writeln!(out, "Alice").unwrap();
        } else {
            writeln!(out, "Bob").unwrap();
        }

        // if bob first
        if m_max >= n_max {
            writeln!(out, "Bob").unwrap();
        } else {
            writeln!(out, "Alice").unwrap();
        }
    }    
}