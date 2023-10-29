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
    // BAD: logic should be correct, (done using the solution) but submission fails
    for _ in 0..scan!(u32) {
        let a = scan!(u32);
        let b = scan!(u32);
        let c = scan!(u32);

        let ae = (0..27).map(|i| a & (1 << i) > 0).vec();
        let be = (0..27).map(|i| b & (1 << i) > 0).vec();
        let ce = (0..27).map(|i| c & (1 << i) > 0).vec();

        let mut carry = false;
        for i in 0..27 {
            let i = 26 - i;
            if carry {
                if ae[i] && be[i] && !ce[i] {
                    carry = false;
                } else {
                    carry = true;
                }
            } else {
                if ae[i] && be[i] && !ce[i] {
                    carry = true;
                } else {
                    carry = false;
                }
            }
        }
        if carry {
            sayln!("no");
        } else {
            sayln!("yes");
        }
    }
}
