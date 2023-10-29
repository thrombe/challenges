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
        let b = scan!(i32) - 1;
        let a = scan!(i32) - 1;
        let mut board = [[0; 8]; 8];

        // [(2, 1), (2, -1), (-2, 1), (-2, -1)]
        // .iter()
        // .map(|(x, y)| (a+x, b+y))
        // .filter(|&(x, y)| x.is_positive() && y.is_positive() && x < 8 && y < 8)
        // .take(2)
        // .for_each(|(x, y)| {
        //     board[y as usize][x as usize] = 2;
        // });

        // [(1, 2), (1, -2), (-1, 2), (-1, -2)]
        // .iter()
        // .map(|(x, y)| (a+x, b+y))
        // .filter(|&(x, y)| x.is_positive() && y.is_positive() && x < 8 && y < 8)
        // .take(1)
        // .for_each(|(x, y)| {
        //     board[y as usize][x as usize] = 2;
        // });

        let hori_rev = a > 3;
        let vert_rev = b > 3;
        let rev_a = if hori_rev { (a - 7).abs() } else { a };
        let rev_b = if vert_rev { (b - 7).abs() } else { b };
        board[rev_b as usize][rev_a as usize] = 1;
        let mut qs = vec![];

        match (rev_a < 1, rev_b < 1) {
            (true, true) => {
                // dbg!();
                qs.push((2, 1));
            }
            (true, false) => {
                // dbg!();
                qs.push((2, 1));
                qs.push((2, -1));
            }
            (false, true) => {
                // dbg!();
                qs.push((1, 2));
                qs.push((-1, 2));
            }
            (false, false) => {
                // dbg!();
                qs.push((-1, 2));
                qs.push((3, -1));
            }
        }
        qs.into_iter()
            .map(|(x, y)| (rev_a + x, rev_b + y))
            // .inspect(|d| {dbg!(d);})
            .filter(|&(x, y)| x >= 0 && y >= 0 && x < 8 && y < 8)
            // .inspect(|d| {dbg!(d);})
            .for_each(|(x, y)| {
                board[y as usize][x as usize] = 2;
            });

        let mut b = board.iter();
        for _ in 0..8 {
            let line = if vert_rev {
                b.next_back().unwrap()
                // b.next().unwrap()
            } else {
                b.next().unwrap()
            };
            let mut l = line.iter();
            for _ in 0..8 {
                let sqr = if hori_rev {
                    l.next_back().unwrap()
                    // l.next().unwrap()
                } else {
                    l.next().unwrap()
                };
                say!("{} ", sqr);
            }
            sayln!("");
        }
        sayln!("");
    }
}
