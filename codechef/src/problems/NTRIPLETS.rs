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
// (prime, power)
fn prime_factorisation<T>(mut n: T) -> Vec<(T, T)>
where
    T: std::ops::SubAssign
        + std::ops::AddAssign
        + std::ops::DivAssign
        + std::ops::Rem<Output = T>
        + From<u8>
        + std::cmp::PartialEq
        + std::cmp::PartialOrd
        + Copy,
{
    let mut factorisation = vec![];
    let mut prime: T = 3.into();
    let mut power: T = 0.into();

    // taking care of 2 in seperate loop to prevent an if condition in the other loop (2 is only even prime)
    while n % 2.into() == 0.into() {
        n /= 2.into();
        power += 1.into();
    }
    if power != 0.into() {
        factorisation.push((2.into(), power));
    }

    while n > 1.into() {
        power = 0.into();
        while n % prime == 0.into() {
            n /= prime;
            power += 1.into();
        }
        if power != 0.into() {
            factorisation.push((prime, power));
        }
        prime += 2.into();
    }
    factorisation
}

// returns primes from 2 upto (not including) n
fn primes_upto<T>(n: T) -> Vec<T>
where
    T: Into<usize> + From<usize> + std::ops::Mul + std::ops::Div + Copy,
{
    let sze = n.into() / 2_usize;
    let sqr = (n.into() as f64).sqrt() as usize + 1;
    let mut nums = vec![false; sze];
    nums[0] = true; // ignoring 1
                    // true is composite and false is potential prime
    let mut primes: Vec<T> = Vec::new();
    primes.push(2_usize.into());
    for i in 1..sqr {
        if nums[i] {
            continue;
        };
        let prime: usize = i * 2_usize + 1;
        primes.push(prime.into());
        let mut j = prime * prime / 2;
        while j < sze {
            nums[j] = true;
            j += prime;
        }
    }
    for i in sqr..sze {
        if !nums[i] {
            primes.push((i * 2_usize + 1).into());
        }
    }
    // println!("{:?}", primes);
    primes
}

fn main() {
    let primes = primes_upto(1000_000_0);
    for _ in 0..scan!(u32) {
        let m = scan!(u32);

        let mut found = false;
        for p in &primes {
            let p = *p as _;
            if p >= m {
                break;
            }
            if m % p == 0 && m != p && p != m / p {
                sayln!("1 {} {}", p, m / p);
                found = true;
                break;
            }
        }
        if !found {
            sayln!("-1");
        }
    }
}
