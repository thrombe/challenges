#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::cell::UnsafeCell;
use std::io::{stdin, stdout, BufWriter, Stdout, Write};
use std::prelude::v1::*;
use std::str::{FromStr, SplitWhitespace};

struct Scanner {
    buffer: SplitWhitespace<'static>,
    leaked: *mut str,
}
impl Scanner {
    fn next<T: std::str::FromStr>(&mut self) -> T {
        unsafe { self.next_raw() }
            .parse()
            .ok()
            .expect("Failed parse")
    }
    unsafe fn next_raw<'a>(&'a mut self) -> &'a str {
        loop {
            if let Some(token) = self.buffer.next() {
                return token;
            }
            let mut input = String::from(Box::from_raw(self.leaked));
            input.clear();
            stdin().read_line(&mut input).expect("Failed read");
            self.leaked = Box::into_raw(input.into_boxed_str());
            let v = &*self.leaked;
            self.buffer = v.split_whitespace();
        }
    }
}
impl Default for Scanner {
    fn default() -> Self {
        let leaked = Box::into_raw(String::new().into_boxed_str());
        let buffer = unsafe { &*leaked }.split_whitespace();
        Self { buffer, leaked }
    }
}
impl Drop for Scanner {
    fn drop(&mut self) {
        let _ = unsafe { Box::from_raw(self.leaked) };
    }
}

thread_local! {
    static SCANNER: UnsafeCell<Scanner> = Scanner::default().into();
    static WRITER: UnsafeCell<BufWriter<Stdout>> = BufWriter::new(stdout()).into();
}
macro_rules! sayln {
    ($($arg:tt)*) => { writeln!(unsafe { WRITER.with(|e| &mut (*e.get())) }, $($arg)*).unwrap(); };
}
macro_rules! say {
    ($($arg:tt)*) => { write!(unsafe { WRITER.with(|e| &mut (*e.get())) }, $($arg)*).unwrap(); };
}
macro_rules! scan {
    ($i:ident) => {
        unsafe { SCANNER.with(|mut e| (*e.get()).next::<$i>()) }
    };
    () => {
        unsafe { SCANNER.with(|mut e| (*e.get()).next()) }
    };

    // using SCANNER after calling this might invalidate this refrence returned by this call. unsafe!!. but this is slightly faster when it is safe to use this
    (&str) => {
        unsafe { SCANNER.with(|mut e| (*e.get()).next_raw()) }
    };
}

fn main() {
    let r = scan!(i32);
    let c = scan!(i32);
    let mut m = (0..r)
        .into_iter()
        .map(|_| (0..c).into_iter().map(|_| scan!(i64)).collect())
        .collect::<Vec<Vec<_>>>();
    let diagonals = {
        let mut ds = vec![];
        for i in 0..c + r {
            let mut d = vec![];
            for j in 0..r {
                if i - j < 0 || i - j >= c {
                    continue;
                }
                let num = m[j as usize][(i - j) as usize];
                d.push(num);
            }
            if d.is_empty() {
                continue;
            }

            let mut nd = vec![];
            let mut s = 0;
            let mut sign = d[0] >= 0;
            for e in d {
                if e > 0 {
                    if sign {
                        s += e;
                    } else {
                        nd.push(s);
                        s = e;
                        sign = !sign;
                    }
                } else if e < 0 {
                    if !sign {
                        s += e
                    } else {
                        sign = !sign;
                        nd.push(s);
                        s = e;
                    }
                } else {
                    s += e;
                }
            }
            nd.push(s);

            let mut curr_max = nd[0];
            let mut g_max = nd[0];
            let mut ei = 0;
            let mut si = 0;
            let mut gmsi = 0;
            for i in 0..nd.len() {
                if nd[i] > nd[i] + curr_max {
                    curr_max = nd[i];
                    si = i;
                } else if nd[i] < nd[i] + curr_max {
                    curr_max += nd[i];
                }

                if curr_max > g_max {
                    g_max = curr_max;
                    ei = i;
                    gmsi = si;
                }
            }
            let s = nd[gmsi..ei + 1]
                .iter()
                .cloned()
                .reduce(|a, b| a + b)
                .unwrap();
            ds.push(s);
        }
        ds
    };

    // dbg!(diagonals);
    sayln!("{}", diagonals.into_iter().max().unwrap());
}
