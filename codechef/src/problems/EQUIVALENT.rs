#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]

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

// ? both main and main2 give TLE on different tests.
// even tho both have same algorithm. just that one uses iterators and stuff and other one dosen't
// and one creates Vec<(_, _)> for prime factorisation, other dosen't
// https://www.codechef.com/viewsolution/75495797
// this guy has same method. but it passes

fn main2() {
    let primes = primes_upto(1000_000u64);
    for _ in 0..scan!(u32) {
        let a = scan!(u64);
        let b = scan!(u64);
        let pfa = prime_factorisation_2(&primes, a);
        let pfb = prime_factorisation_2(&primes, b);
        if pfa.len() != pfb.len() {
            sayln!("NO");
            continue;
        }
        let x = pfa[0].1;
        let y = pfb[0].1;
        if pfa.iter().zip(pfb.iter()).all(|(a, b)| a.0 == b.0) {
            if pfa
                .into_iter()
                .zip(pfb.into_iter())
                .map(|(a, b)| (a.1, b.1))
                .all(|(a, b)| a * y == b * x)
            {
                sayln!("YES");
                continue;
            }
        }
        sayln!("NO");
    }
}

fn main() {
    let primes = primes_upto(1000_000u64);
    'outer: for _ in 0..scan!(u32) {
        let mut a = scan!(u64);
        let mut b = scan!(u64);
        let mut r: Option<(u64, u64)> = None;
        for &p in &primes {
            let mut c1 = 0;
            while a % p == 0 {
                a /= p;
                c1 += 1;
            }
            let mut c2 = 0;
            while b % p == 0 {
                b /= p;
                c2 += 1;
            }
            if c1 > 0 || c2 > 0 {
                if c1 * c2 == 0 {
                    sayln!("NO");
                    continue 'outer;
                }
                if let Some(r) = r {
                    if r.0 * c2 != r.1 * c1 {
                        sayln!("NO");
                        continue 'outer;
                    }
                } else {
                    r = Some((c1, c2));
                }
            }
        }
        sayln!("YES");
    }
}

fn prime_factorisation_2<T>(primes: &Vec<T>, mut n: T) -> Vec<(T, T)>
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
    let mut fac = vec![];
    for &p in primes {
        let mut cnt = 0.into();
        while n % p == 0.into() {
            n /= p;
            cnt += 1.into();
        }
        if cnt > 0.into() {
            fac.push((p, cnt));
        }
        if n == 1.into() {
            break;
        }
    }
    fac
}

// returns primes from 2 upto (not including) n
fn primes_upto<T>(n: T) -> Vec<T>
where
    T: Into<u64> + From<u64> + std::ops::Mul + std::ops::Div + Copy,
{
    let sze = n.into() / 2u64;
    let sqr = (n.into() as f64).sqrt() as u64 + 1;
    let mut nums = vec![false; sze as usize];
    nums[0] = true; // ignoring 1
                    // true is composite and false is potential prime
    let mut primes: Vec<T> = Vec::new();
    primes.push(2u64.into());
    for i in 1..sqr {
        if nums[i as usize] {
            continue;
        };
        let prime: u64 = i * 2u64 + 1;
        primes.push(prime.into());
        let mut j = prime * prime / 2;
        while j < sze {
            nums[j as usize] = true;
            j += prime;
        }
    }
    for i in sqr..sze {
        if !nums[i as usize] {
            primes.push((i * 2u64 + 1).into());
        }
    }
    primes
}
