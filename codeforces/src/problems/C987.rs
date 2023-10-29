
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
    let mut scanner = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let n = scanner.next::<usize>();
    let s = (0..n).map(|_| scanner.next::<i32>()).collect::<Vec<i32>>();
    let c = (0..n).map(|_| scanner.next::<i32>()).collect::<Vec<i32>>();

    let ans = (1..n-1)
    .filter_map(|j| {
        let ci = (0..j)
        .filter(|&i| s[i] < s[j])
        .map(|i| c[i])
        .min();
        if ci.is_none() {return None}

        let ck = (j+1..n)
        .filter(|&i| s[i] > s[j])
        .map(|i| c[i])
        .min();
        
        ck.map(|e| ci.unwrap() + c[j] + e)
    })
    .min()
    .unwrap_or(-1);
    
    writeln!(out, "{ans}").unwrap();
}

