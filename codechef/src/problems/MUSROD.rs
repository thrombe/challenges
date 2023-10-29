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

// assume 2 elements at i th and i+1 th places
// let 'as' be sum Aj till i-1
// let sum be the required sum
// then we try to figure out if Ai should be placed first or A(i+1)
// if Ai is placed first => sum += (as + Ai)*B(i+1) + as*Bi => sum += as*(Bi + B(i+1)) + Ai*B(i+1)
// if A(i+1) is placed first => sum += (as + A(i+1))*Bi + as*B(i+1) => sum += as(Bi + B(i+1)) + A(i+1)*Bi
// we can see, sum would be greater in case 1 if Ai*B(i+1) > A(i+1)*Bi ==> Ai/Bi > A(i+1)/B(i+1)
// so we just sort the elements in descending order of Ai/Bi and calculate the thing
fn main() {
    // BAD: for some reason, this code fails. but same converted to python passes. no idea why
    for _ in 0..scan!(u32) {
        let n = scan!(u32);
        let a = (0..n).map(|_| scan!(u32)).vec();
        let b = (0..n).map(|_| scan!(u32)).vec();

        let mut v = a.iter().cloned().zip(b.iter().cloned()).vec();
        v.sort_by(|a, b|
            // (b.0 as f64/b.1 as f64)
            // .partial_cmp(&(a.0 as f64/a.1 as f64)).unwrap()
            (b.0*a.1).cmp(&(a.0*b.1)));
        let mut s = 0;
        let mut sum = 0;
        for e in v {
            sum += s * e.1;
            s += e.0;
        }
        sayln!("{}", sum);
    }
}
