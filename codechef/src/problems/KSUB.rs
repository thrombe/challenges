#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::cell::UnsafeCell;
use std::io::{stdin, stdout, BufWriter, Stdout, Write};
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
        let k = scan!(u64);
        let a = (0..n).into_iter().map(|_| scan!(u64)).collect::<Vec<_>>();
        let ogcd = a.iter().cloned().reduce(gcd).unwrap();

        let mut cnt = 0;
        let mut g = 0;
        a.into_iter().for_each(|e| {
            g = gcd(g, e);
            if g == ogcd {
                // ? avg of gcd of all k subarrays == ogcd. how is that satisfied here?
                g = 0;
                cnt += 1;
            }
        });
        if cnt >= k {
            sayln!("YES");
        } else {
            sayln!("NO");
        }
    }
}

/// euclids gcd algorithm
///
/// let g = gcd(a, b)
/// then g divides a and g divides b
/// so g has to divide |a-b| => g divides a%b (let a > b)
fn gcd<T>(mut a: T, mut b: T) -> T
where
    T: std::ops::Rem<Output = T> + std::cmp::PartialOrd + Copy + From<u64>,
{
    if b > a {
        let tmp = a;
        a = b;
        b = tmp;
    }
    if b == 0_u64.into() {
        return a;
    }
    if a == b {
        return a;
    }
    gcd(a % b, b)
}
