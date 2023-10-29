
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

    let mut f = [0, 0, 0];
    for (i, ele) in (0..s.next::<usize>()*3).map(|_| s.next::<i32>()).enumerate() {
        f[i%3] += ele;
        // writeln!(out, "{i}").ok();
        // out.flush();
    }
    if f != [0, 0, 0] {
        writeln!(out, "NO").ok();
    } else {
        writeln!(out, "YES").ok();
    }
}