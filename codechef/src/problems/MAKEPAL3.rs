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
        let n = scan!();
        let a = (0..n).map(|_| scan!(i64)).vec();

        let mut b = a.iter().cloned().rev().take(n / 2).rev().vec();
        let mut a = a
            .iter()
            .cloned()
            .take(n / 2)
            .rev()
            .zip(b.into_iter())
            .map(|(i, j)| i - j)
            // .rev()
            .vec();
        // dbg!(&a, &b);

        // let mut v = a.iter().cloned().peekable();
        let mut count = 0;

        while let Some(&p) = a.last() {
            match p.signum() {
                1 => {
                    let v = a.iter().rev().cloned().take_while(|&e| e > 0);
                    let k = v.clone().min().unwrap();
                    let p = v.count();
                    let m = a.len();
                    count += k;
                    for i in 0..p {
                        a[m - 1 - i] -= k;
                    }
                    if a[m - 1] == 0 {
                        let _ = a.pop();
                    }
                }
                -1 => {
                    let v = a.iter().rev().cloned().take_while(|&e| e < 0);
                    let k = v.clone().max().unwrap();
                    let p = v.count();
                    let m = a.len();
                    count += -k;
                    for i in 0..p {
                        a[m - 1 - i] -= k;
                    }
                    if a[m - 1] == 0 {
                        let _ = a.pop();
                    }
                }
                0 => {
                    let _ = a.pop();
                }
                _ => unreachable!(),
            }
        }
        sayln!("{}", count);
    }
}
