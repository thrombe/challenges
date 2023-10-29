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

    /*
    we can basically move any item from one index to another by choosing the subarrays properly
    so we count how many pairs have equal numbers (take min over 1, 1 paris and 0, 0 pair counts)
    and mod it by 2 to find whoose chance it will be after the last move
    as in each move, one of these paris can be eliminated by moving a 1 from 11 between a 00 pair
    (assuming number of ones is greater than number of zeros)
    */
    for _ in 0..scan!(u32) {
        let n = scan!(usize);
        let a = (0..n)
            .into_iter()
            .map(|_| scan!(u32))
            .map(|b| b == 1)
            .collect::<Vec<_>>();

        let mut zeros = 0;
        let mut ones = 0;
        a.iter()
            .cloned()
            .zip(a.iter().cloned().skip(1).chain(std::iter::once(a[0])))
            .filter(|&(a, b)| a == b)
            .map(|(a, _)| a)
            .for_each(|a| {
                if a {
                    ones += 1;
                } else {
                    zeros += 1;
                }
            });

        if ones.min(zeros) % 2 == 0 {
            sayln!("Bob");
        } else {
            sayln!("Alice");
        }
    }
}
