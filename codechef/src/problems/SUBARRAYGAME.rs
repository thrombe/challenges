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
        let diff = a
            .iter()
            .cloned()
            .zip(a.iter().cloned().skip(1))
            .map(|(a, b)| a - b)
            .collect::<Vec<_>>();
        let mut lens = vec![];
        let mut cnt = 0;
        let mut pve = diff[0].is_positive();
        for &i in diff.iter().skip(1) {
            if i > 0 {
                if pve {
                    cnt += 1;
                } else {
                    pve = true;
                    if cnt != 0 {
                        lens.push(cnt);
                    }
                    cnt = 0;
                }
            } else if i < 0 {
                if pve {
                    pve = false;
                    if cnt != 0 {
                        lens.push(cnt);
                    }
                    cnt = 0;
                } else {
                    cnt += 1;
                }
            }
        }
        if cnt != 0 {
            lens.push(cnt);
        }
        // dbg!(&lens);

        // let gt2 = lens.iter().cloned().filter(|&e| e > 1).count();

        // match lens.iter().cloned().max() {
        //     Some(m) => {
        //         let chances = lens.len();
        //         if chances%2 == 0 { // ends with p2
        //             if m > 1 {
        //                 sayln!("Bob");
        //             } else {
        //                 sayln!("Alice");
        //             }
        //         } else { // ends with p1
        //             if m > 1 {
        //                 sayln!("Alice");
        //             } else {
        //                 sayln!("Bob");
        //             }
        //         }
        //     }
        //     None => {
        //         sayln!("Bob");
        //     }
        // }

        // apparently it is equivalent to Nim: https://en.wikipedia.org/wiki/Nim
        // can be solved just by performing xor across all items in "lens"
        if lens
            .into_iter()
            .reduce(|a, b| a ^ b)
            .map(|e| e == 0)
            .unwrap_or(true)
        {
            sayln!("Bob");
        } else {
            sayln!("Alice");
        }
    }
}
