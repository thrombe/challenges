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
/// euclids gcd algorithm
///
/// let g = gcd(a, b)
/// then g divides a and g divides b
/// so g has to divide |a-b| => g divides a%b (let a > b)
fn gcd<T>(mut a: T, mut b: T) -> T
where
    T: std::ops::Rem<Output = T> + std::cmp::PartialOrd + Copy + From<u8>,
{
    if b > a {
        let tmp = a;
        a = b;
        b = tmp;
    }
    if b == 0.into() {
        return a;
    }
    if a == b {
        return a;
    }
    gcd(a % b, b)
}

fn main() {
    for _ in 0..scan!(u32) {
        let x = scan!(i64);
        let y = scan!(i64);

        // n + x = qy, n + y = px
        // (q+1)y = (p+1)x
        // x/y = (q+1)/(p+1)
        let g = gcd(x, y);
        // dbg!(x, y, g);
        let p = y / g;
        let q = x / g;
        // let p = y;
        // let q = x;

        let qy = q * y;
        let mut qq = ((x + y) / qy + 1) * qy;
        // let n = (q-1)*y - x;
        let n = qq - x - y;
        // dbg!(x, y, g, p, q, qy, qq, "");
        sayln!("{}", n);

        // match (p==1, q==1) {
        //     (true, true) => {
        //         sayln!("0");
        //     }
        //     (true, false) | (false, false) => {
        //         let qy = q*y;
        //         let mut qq = ((x+y)/qy + 1)*qy;
        //         // let n = (q-1)*y - x;
        //         let n = qq - x - y;
        //         dbg!();
        //         sayln!("{}", n);
        //     }
        //     (false, true) => {
        //         let px = p*x;
        //         let mut pp= ((x+y)/px + 1)*px;
        //         // let n = (p-1)*x - y;
        //         let n = pp - x - y;
        //         sayln!("{}", n);
        //     }
        // }
    }
}
