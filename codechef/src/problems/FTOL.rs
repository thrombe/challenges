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
        let n = scan!(u64);
        let m = scan!(u64);
        let k = scan!(u64);

        let mut specials = (0..k).map(|_| (scan!(u64), scan!(u64))).vec();
        specials.push((0, 0));
        specials.sort_by(|a, b| {
            // ((n-a.0).pow(2) + (m-a.1).pow(2))
            // .cmp(&((n-b.0).pow(2) + (m-b.1).pow(2)))
            a.0.cmp(&b.0) // sort it by x coord
        });
        // let specials = std::iter::once((0, 0))
        
        // .chain(specials.into_iter())
        // .into_iter()
        // .chain()
        // .rev()
        // .enumerate()
        // .vec();

        // i couldn't do longest increasing subsequence :(
        let mut coords = specials[0];
        let mut count = 0;
        for &cds in &specials[1..] {
            if coords.1 > cds.1 {
                count += 1;
                if coords.0 == cds.0 {
                    coords.1 = coords.1.min(cds.1);
                } else {
                    coords = cds;
                }
            } else {

            }
        }
    }
}

fn main2() {
    for _ in 0..scan!(u32) {
        let n = scan!(u64);
        let m = scan!(u64);
        let k = scan!(u64);

        let mut specials = (0..k).map(|_| (scan!(u64), scan!(u64))).vec();
        specials.sort_by(|a, b| {
            ((n - a.0).pow(2) + (m - a.1).pow(2)).cmp(&((n - b.0).pow(2) + (m - b.1).pow(2)))
        });
        let specials = specials
            .into_iter()
            .chain(std::iter::once((1, 1)))
            .rev()
            .enumerate()
            .vec();

        let mut dp = vec![0; (k + 1) as _];
        // for i in (0..(k+1) as _).rev() {
        //     dp[i] = r(&mut dp, &specials, i, n, m);
        // }
        // sayln!("{}", dp[0]);
        sayln!("{}", r(&mut dp, &specials, 0, n, m) + 1);
    }
}

fn r(dp: &mut [u64], specials: &[(usize, (u64, u64))], index: usize, n: u64, m: u64) -> u64 {
    if dp[index] != 0 {
        return dp[index];
    }
    let (j, (x, y)) = specials[index];
    let ans = if let Some(v) = specials[(index + 1)..]
        .iter()
        .cloned()
        .filter(|&(i, (p, q))| p > x && q > y)
        .map(|(i, (p, q))| {
            // dbg!(p, q, x, y, index);
            (p-x) +
        (q-y) +
        r(dp, specials, i, n, m)
        // - if (p-x) != 0 && (q-y) != 0 {1} else {0}
        - if (n-p) != 0 && (m-q) != 0 {1} else {0}
            // - 1
        })
        .min()
    {
        v
    } else {
        (n - x) + (m - y) - if (n - x) != 0 && (m - y) != 0 { 1 } else { 0 }
    };

    dp[j] = ans;
    ans
}
