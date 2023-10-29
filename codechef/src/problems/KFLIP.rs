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

    // using SCANNER calling this might invalidate this refrence returned by this call. unsafe!!. but this is slightly faster when it is safe to use this
    (&str) => {
        unsafe { SCANNER.with(|mut e| (*e.get()).next_raw()) }
    };
}

fn main() {
    let mo = 1000_000_000 + 7;
    for _ in 0..scan!(u32) {
        let n = scan!(u32);
        let k = scan!(u32);
        let _ = scan!(&str);
        if n == k {
            sayln!("2");
        } else {
            if k % 2 == 0 {
                // if k is even, it is possible to flip 2 bits in k-1 operations.
                // consider k+1 bits. now choose first bit in each operation and then for the rest of k bits, we have to choose k-1 more bits
                // which is just the case where k-1 is odd. so proceeding in the same way.
                let mut ans = 1u64;
                for _ in 0..n - 1 {
                    ans = (ans << 1) % mo;
                }
                sayln!("{}", ans);
            } else {
                // if k is odd, it is possible to flip a single bit in k operations.
                // consider first k+1 bits. to flip the first bit, in each of the k operations,
                // flip the first bit, and from the rest of the k bits, choose a unique subset of bits (k-1 bits) in each operation
                // so the first bit is flipped k times (odd) but the rest are flipped k-1 times (even).
                let mut ans = 1u64;
                for _ in 0..n {
                    ans = (ans << 1) % mo;
                }
                sayln!("{}", ans);
            }
        }
    }
}
