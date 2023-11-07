#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::io::{ BufWriter, stdin, stdout, Write, Stdout };
use std::iter::Map;
use std::str::{ SplitWhitespace, FromStr };
use std::cell::UnsafeCell;
use std::collections::{ HashMap, HashSet, BinaryHeap };

struct Scanner {
    buffer: SplitWhitespace<'static>,
    leaked: *mut str,
}
impl Scanner {
    fn next<T: std::str::FromStr>(&mut self) -> T {
        unsafe { self.next_raw() }.parse().ok().expect("Failed parse")
    }
    unsafe fn next_raw<'a>(&'a mut self) -> &'a str {
        loop {
            if let Some(token) = self.buffer.next() { return token; }
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
    fn drop(&mut self) { let _ = unsafe { Box::from_raw(self.leaked) }; }
}
trait Vek<T>: Iterator<Item=T> {
    fn vec(self) -> Vec<T>;
}
impl<T, P: Iterator<Item=T>> Vek<T> for P {
    fn vec(self) -> Vec<T> { self.collect() }
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
    ($i:ident) => { unsafe { SCANNER.with(|mut e| (*e.get()).next::<$i>()) } };
    () => { unsafe { SCANNER.with(|mut e| (*e.get()).next()) } };
    
    // using SCANNER after calling this might invalidate this refrence returned by this call. unsafe!!. but this is slightly faster when it is safe to use this
    (&str) => { unsafe { SCANNER.with(|mut e| (*e.get()).next_raw()) } };
}

#[derive(Clone, Debug)]
struct Cust((usize, usize));
impl Cust {
    fn g(&self) -> usize {
        ((self.0.1 + self.0.0)/2) + (self.0.1 - self.0.0)*200000
    }
}
impl PartialEq for Cust {
    fn eq(&self, other: &Self) -> bool {
        self.g().eq(&other.g())
    }
}
impl PartialOrd for Cust {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.g().partial_cmp(&other.g())
    }
}
impl Eq for Cust {}
impl Ord for Cust {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.g().cmp(&other.g())
    }
}
fn main() {
    for _ in 0..scan!(u32) {
        let n = scan!(usize);
        let m = scan!(usize);
        let mut a = (0..n).map(|_| scan!(usize)).collect::<Vec<_>>();
        let mut b = (0..m).map(|_| scan!(usize)).collect::<Vec<_>>();
        b.sort();
        let mut ater = a.iter().peekable();

        let mut ans: Vec<usize> = vec![];
        loop {
            match (ater.peek(), b.last()) {
                (None, None) => {
                    break;
                },
                (None, Some(q)) => {
                    ans.push(*q);
                    let _ = b.pop();
                },
                (Some(p), None) => {
                    ans.push(**p);
                    let _ = ater.next();
                },
                (Some(p), Some(q)) => {
                    if **p > *q {
                        ans.push(**p);
                        let _ = ater.next();
                    } else if *q >= **p {
                        ans.push(*q);
                        let _ = b.pop();
                    }
                },
            }
        }
        for p in ans {
            say!("{} ", p);
        }
        sayln!();
    }
}
