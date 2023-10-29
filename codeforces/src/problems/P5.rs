
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

    let _n = s.next::<u64>();
    let dos = s.next::<String>();

    if dos.len() < 2 {
        if dos == "." {
            writeln!(out, "1").unwrap();
        } else {
            writeln!(out, "0").unwrap();
        }
        return;
    }

    let mut count = 0;
    let mut line = dos.chars().enumerate().filter(|(_, c)| *c == 'L' || *c == 'R');
    let prev = line.next();
    if prev.is_none() {
        writeln!(out, "{}", dos.len()).unwrap();
        return;
    }
    let mut prev = prev.unwrap();
    let current = line.next();
    if current.is_none() {
        if prev.1 == 'L' {
            writeln!(out, "{}", dos.len()-prev.0-1).unwrap();
        } else {
            writeln!(out, "{}", prev.0).unwrap();
        }
        return
    }
    let mut current = current.unwrap();

    if prev.1 == 'R' {
        count += prev.0;
    }

    if prev.1 == 'R' {
        count += (current.0 - prev.0 -1)%2;
    } else {
        count += current.0 - prev.0 -1;
    }

    for k in line {
        prev = current;
        current = k;
        if prev.1 == 'R' {
            count += (current.0 - prev.0 -1)%2;
        } else {
            count += current.0 - prev.0 -1;
        }
    }

    if current.1 == 'L' {
        count += dos.len()-current.0-1;
    }

    writeln!(out, "{count}").unwrap();
}