#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]

use std::cell::UnsafeCell;
use std::io::{stdin, stdout, BufWriter, Stdout, Write};
use std::iter::Map;
use std::str::{FromStr, SplitWhitespace};
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

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
trait Vek<T>: Iterator<Item = T> {
    fn vec(self) -> Vec<T>;
}
impl<T, P: Iterator<Item = T>> Vek<T> for P {
    fn vec(self) -> Vec<T> {
        self.collect()
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
    for _ in 0..scan!(u32) {
        let n = scan!(u32);
        let mut a = (0..2 * n).map(|_| scan!(u32)).vec();

        let mut m = HashMap::new();
        for &e in &a {
            if let Some(k) = m.insert(e, 1) {
                m.insert(e, k + 1);
            }
        }
        let mut v = m.iter().map(|e| (*e.0, *e.1)).vec();
        v.sort_by(|a, b| b.1.cmp(&a.1));
        // dbg!(&v);

        if v[0].1 == 2 * n {
            sayln!("no");
            continue;
        }

        if v[0].1 > n {
            let k = v[0].1;
            if k % 2 == 0 {
                let m = 2 * n - k;
                if (k - m) / 2 <= m {
                    sayln!("yes");
                } else {
                    sayln!("no");
                }
            } else {
                let m = 2 * n - k;
                if (k - 1 - m) / 2 <= m {
                    sayln!("yes");
                } else {
                    sayln!("no");
                }
            }
        } else {
            sayln!("yes");
        }
    }
}

fn main2() {
    'outer: for _ in 0..scan!(u32) {
        let n = scan!(u32);
        let mut a = (0..2 * n).map(|_| scan!(u32)).vec();
        // a.sort();

        let mut m = HashMap::new();
        for &e in &a {
            if let Some(k) = m.insert(e, 0) {
                m.insert(e, k + 1);
            }
        }

        let mut s = HashSet::new();

        let mut v = m.iter().map(|e| (*e.0, *e.1)).vec();
        v.sort_by(|a, b| a.0.cmp(&b.0));
        let mut count = 0;
        for i in 0..v.len() {
            if i < v.len() / 2 {
                let m = v[v.len() - 1 - i].0 - v[i].0;
                if !s.contains(&m) {
                    s.insert(m);
                }
                let k = v[v.len() - i - 1].1.min(v[i].1).min(n);
                let l = v.len();
                v[l - 1 - i].1 -= k;
                v[i].1 -= k;
                if count > n {
                    sayln!("yes");
                    continue 'outer;
                }
            } else {
                continue;
            }
        }
        sayln!("no");
    }
}
