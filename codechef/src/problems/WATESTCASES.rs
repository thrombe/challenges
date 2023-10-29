#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::io::{stdin, stdout, BufWriter, Write};
use std::prelude::v1::*;

#[derive(Default)]
struct Scanner {
    buffer: Vec<String>,
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
        ($i:ty) => {
            scanner.next::<$i>()
        };
        () => {
            scanner.next()
        };
    }

    (0..scan!(u32)).into_iter().for_each(|_| {
        let n = scan!(u32);
        let s = (0..n).map(|_| scan!(u32)).collect::<Vec<_>>();
        let ans = scan!(String)
            .chars()
            .map(|c| c == '1')
            .enumerate()
            .filter(|(i, e)| !*e)
            .map(|(i, _)| s[i])
            .min()
            .unwrap();
        sayln!("{}", ans);
    });
}
