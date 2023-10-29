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
        let q = scan!();
        let a = (0..n).map(|_| scan!(u64)).vec();
        let counts = (0..60)
            .map(|i| {
                let v = a.iter().copied().map(|e| (e & 1 << i) > 0).enumerate();
                let has = v.clone().filter(|&(i, e)| e).map(|(i, _)| i).vec();
                let hasnt = v.filter(|&(i, e)| !e).map(|(i, _)| i).vec();
                (has, hasnt)
            })
            .vec();
        // dbg!(&counts[..2]);

        for _ in 0..q {
            let k = scan!(usize);
            let l1 = scan!(usize) - 1;
            let r1 = scan!(usize) - 1;
            let l2 = scan!(usize) - 1;
            let r2 = scan!(usize) - 1;

            let (has, hasnt) = &counts[k];
            let i1 = match has.binary_search(&l1) {
                Ok(i) => i,
                Err(i) => i,
            };
            let i2 = match has.binary_search(&r1) {
                Ok(i) => i + 1,
                Err(i) => i,
            };
            let j1 = match hasnt.binary_search(&l2) {
                Ok(i) => i,
                Err(i) => i,
            };
            let j2 = match hasnt.binary_search(&r2) {
                Ok(i) => i + 1,
                Err(i) => i,
            };
            // dbg!(&has[i1..i2]);
            // dbg!(&hasnt[j1..j2]);

            let mut c = (i2 - i1) * (j2 - j1);
            let i1 = match hasnt.binary_search(&l1) {
                Ok(i) => i,
                Err(i) => i,
            };
            let i2 = match hasnt.binary_search(&r1) {
                Ok(i) => i + 1,
                Err(i) => i,
            };
            let j1 = match has.binary_search(&l2) {
                Ok(i) => i,
                Err(i) => i,
            };
            let j2 = match has.binary_search(&r2) {
                Ok(i) => i + 1,
                Err(i) => i,
            };
            // dbg!(&hasnt[i1..i2]);
            // dbg!(&has[j1..j2]);

            c += (i2 - i1) * (j2 - j1);
            sayln!("{}", c);
        }
    }
}
