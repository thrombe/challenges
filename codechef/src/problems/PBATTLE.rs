#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::cell::UnsafeCell;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::io::{stdin, stdout, BufWriter, Stdout, Write};
use std::iter::Map;
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
        let mut s = vec![HashSet::new(); n];
        let mut a = (0..n).map(|_| scan!(u32)).vec().into_iter();
        let mut b = (0..n).map(|_| scan!(u32)).vec().into_iter();
        let mut t = (0..n)
            .map(|i| (i, a.next().unwrap(), b.next().unwrap()))
            .vec();

        // dbg!(&t);
        t.sort_by(|a, b| a.1.cmp(&b.1));
        t.iter().enumerate().for_each(|(j, &(i, _, _))| {
            (0..j).map(|l| t[l].0).for_each(|k| {
                s[i].insert(k);
            })
        });

        t.sort_by(|a, b| a.2.cmp(&b.2));
        t.iter().enumerate().for_each(|(j, &(i, _, _))| {
            (0..j).map(|l| t[l].0).for_each(|k| {
                s[i].insert(k);
            })
        });

        // dbg!(&s);

        let s = s.into_iter().map(|e| e.len()).vec();

        // dbg!(&s);

        let ans = s
            .iter()
            .rev()
            .zip(s.iter().rev().skip(1))
            .take_while(|(a, b)| **a == **b)
            .count()
            + 1;

        sayln!("{}", ans);
    }
}
