#![allow(dead_code)]

use std::io::{BufWriter, stdin, stdout, Write};
 
#[derive(Default)]
struct Scanner {
    buffer: Vec<String>
}
impl Scanner {
    fn next(&mut self) -> String {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token;
            }
            let mut input = String::new();
            stdin().read_line(&mut input).expect("Failed read");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }
}

// simpler solution
/*
pop elements and set them at first, last index alternatively
if odd start with index 0 else start with index code,len()
*/


fn main() {
    let mut s = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    s.next(); // no need of length
    let code = s.next().chars().collect::<Vec<char>>();
    let mut left = code.len()%2 == 0;
    let mut p = code.len()/2 - if left {0} else {0};
    // let mut k;
    let mut indices = vec![];
    for _ in 0..code.len() {
        if left {
            indices.push(p-1);
            // k = code[p-1];
            p -= 1;
        } else {
            indices.push(code.len()-p-1);
            // k = code[code.len()-p-1];
        }
        left = !left;
        // write!(out, "{k}").unwrap();
    }
    
    
    let mut result = vec!['a';code.len()];
    for i in 0..code.len() {
        let c = code[i];
        result[indices[i]] = c;
    }
    write!(out, "{}", result.into_iter().collect::<String>()).unwrap();
    // write!(out, "{indices:#?}").unwrap();

}