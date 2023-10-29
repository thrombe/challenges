
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

    let mut xor = 0;
    let mut indices = vec![];
    let mut mat = (0..n).map(|_| {
        let m = (0..m).map(|_| s.next::<u64>()).collect::<Vec<u64>>();
        xor = xor^m[0];
        indices.push(0);
        m
    }).collect::<Vec<Vec<u64>>>();

    while xor == 0 {
        let m = match mat.pop() {
            Some(p) => p,
            None => {
                writeln!(out, "NIE").unwrap();
                return;
            },
        };

        for i in 1..m.len() {
            if m[0] != m[i] { // even a single element change will make xor non 0
                indices[mat.len()] = i;
                xor = !xor; // just to end main while loop
                break;
            }
        }
    }
    writeln!(out, "TAK").unwrap();
    indices.iter().for_each(|j| write!(out, "{} ", j+1).unwrap());
}