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
        ($i:ident) => {
            scanner.next::<$i>()
        };
        () => {
            scanner.next()
        };
    }

    (0..scan!(u64)).into_iter().for_each(|_| {
        let n = scan!(u64);
        let ans = n
            * (0..n)
                .into_iter()
                .map(|_| scan!(u64))
                .reduce(|g, e| gcd(g, e))
                .unwrap();
        sayln!("{}", ans);
    });
}

/// euclids gcd algorithm
///
/// let g = gcd(a, b)
/// then g divides a and g divides b
/// so g has to divide |a-b| => g divides a%b (let a > b)
fn gcd<T>(mut a: T, mut b: T) -> T
where
    T: std::ops::Rem<Output = T> + std::cmp::PartialOrd + Copy + From<u32>,
{
    if b > a {
        let tmp = a;
        a = b;
        b = tmp;
    }
    if b == 0_u32.into() {
        return a;
    }
    if a == b {
        return a;
    }
    gcd(a % b, b)
}
