#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::io::{BufWriter, stdin, stdout, Write};
use std::prelude::v1::*;
 
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
    let writer = &mut BufWriter::new(stdout());
    macro_rules! sayln {
        ($($arg:tt)*) => (
            writeln!(writer, $($arg)*).unwrap();
        );
    }
    macro_rules! say {
        ($($arg:tt)*) => (
            write!(writer, $($arg)*).unwrap();
        );
    }
    macro_rules! scan {
        ($i:ident) => {
            scanner.next::<$i>()
        };
        () => {
            scanner.next()
        };
    }


    
}