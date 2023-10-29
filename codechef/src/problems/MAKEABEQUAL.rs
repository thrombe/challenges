#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::io::{stdin, stdout, BufWriter, Write};
use std::prelude::v1::*;
use std::str::{FromStr, SplitWhitespace};

struct Scanner {
    buffer: SplitWhitespace<'static>,
    leaked: *mut str,
}
impl Scanner {
    fn next<T: std::str::FromStr>(&mut self) -> T {
        self.next_raw().parse().ok().expect("Failed parse")
    }

    fn next_raw<'a>(&'a mut self) -> &'a str {
        loop {
            if let Some(token) = self.buffer.next() {
                return token;
            }
            let mut input = unsafe { String::from(Box::from_raw(self.leaked)) };
            input.clear();
            stdin().read_line(&mut input).expect("Failed read");
            self.leaked = Box::into_raw(input.into_boxed_str());
            let v = unsafe { &*self.leaked };
            self.buffer = v.split_whitespace();
        }
    }
}
impl Default for Scanner {
    fn default() -> Self {
        let input = String::new();
        let leaked = Box::into_raw(input.into_boxed_str());
        let v = unsafe { &*leaked };
        let buffer = v.split_whitespace();
        Self { buffer, leaked }
    }
}
impl Drop for Scanner {
    fn drop(&mut self) {
        let _unleak = unsafe { Box::from_raw(self.leaked) };
    }
}

fn main() {
    let mut scanner = Scanner::default();
    let writer = &mut BufWriter::new(stdout());
    macro_rules! sayln {
        ($($arg:tt)*) => {
            writeln!(writer, $($arg)*).unwrap();
        };
    }
    macro_rules! say {
        ($($arg:tt)*) => {
            write!(writer, $($arg)*).unwrap();
        };
    }
    macro_rules! scan {
        ($i:ident) => {
            scanner.next::<$i>()
        };
        (&str) => {
            scanner.next_raw()
        };
        () => {
            scanner.next()
        };
    }

    for _ in 0..scan!(u32) {
        let n = scan!(u32);
        let a = (0..n).into_iter().map(|_| scan!(i64)).collect::<Vec<_>>();
        let b = (0..n).into_iter().map(|_| scan!(i64)).collect::<Vec<_>>();

        let mut count = 0;
        let s = a
            .into_iter()
            .zip(b.into_iter())
            .inspect(|(a, b)| count += (a - b).abs())
            .map(|(a, b)| a - b)
            .sum::<i64>();
        if s == 0 {
            sayln!("{}", count / 2);
        } else {
            sayln!("-1");
        }
    }
}
