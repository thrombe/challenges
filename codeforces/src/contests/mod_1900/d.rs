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
/// euclids gcd algorithm
///
/// let g = gcd(a, b)
/// then g divides a and g divides b
/// so g has to divide |a-b| => g divides a%b (let a > b)
fn gcd<T>(mut a: T, mut b: T) -> T 
where T: std::ops::Rem<Output=T> + std::cmp::PartialOrd + Copy + From<u64>,
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
    gcd(a%b, b)
}

fn main() {
    let mut gcds = HashMap::<(u64, u64), u64>::new();
    for _ in 0..scan!(u32) {
        let n = scan!(u64);
        let mut a = (0..n).map(|_| scan!(u64)).collect::<Vec<_>>();
        let mut freq = HashMap::<u64, u64>::new();
        for e in a.iter().copied() {
            match freq.get_mut(&e) {
                Some(v) => {
                    *v += 1;
                },
                None => {
                    freq.insert(e, 1);
                },
            }
        }
        a.sort();

        let mut b = freq.iter().map(|a| (*a.0, *a.1)).collect::<Vec<_>>();
        b.sort_by(|a, b| a.0.cmp(&b.0));

        dbg!(&b);

        let mut ans = 0;
        let mut remaining = n;
        for i in 0..b.len() {
            let (mut p, mut pc) = b[i];
            remaining -= pc;
            ans += p * (remaining) * pc * (pc+1) / 2;
            for j in i+1..b.len() {
                // for k in j+1..n {
                    let (mut q, mut qc) = b[j];
                    remaining -= qc;
                    // let mut r = a[k];
                    // if p > q {
                    //     (p, q) = (q, p);
                    //     (pc, qc) = (qc, pc);
                    // }
                    // (p, q) = (p.min(q), p.max(q));
                    // (p, r) = (p.min(r), p.max(r));
                    // (q, r) = (q.min(r), q.max(r));
                    // dbg!(p, q, r);

                    match gcds.get(&(p, q)) {
                        Some(&g) => {
                            // dbg!(g, remaining, pc * (pc+1) * qc * (qc+1) / 4);
                            // ans += g * (n - j - 1);
                            // ans += g * (remaining);
                            ans += g * (remaining) * pc * (pc+1) * qc * (qc+1) / 4;
                            if remaining == 0 {
                                
                            }
                            // if remaining > 0 {
                            //     ans += g * (remaining) * pc * (pc+1) * qc * (qc+1) / 4;
                            // } else {
                            //     ans += g * pc * (pc+1) * qc * (qc+1) / 4;
                            // }
                        }
                        None => {
                            let g = gcd(p, q);
                            dbg!(g, remaining, pc * (pc+1) * qc * (qc+1) / 4);
                            gcds.insert((p, q), g);
                            // ans += g * (remaining) * pc * qc;
                            ans += g * (remaining) * pc * (pc+1) * qc * (qc+1) / 4;
                            // if remaining > 0 {
                            //     ans += g * (remaining) * pc * (pc+1) * qc * (qc+1) / 4;
                            // } else {
                            //     ans += g * pc * (pc+1) * qc * (qc+1) / 4;
                            // }
                        }
                    }
                // }
            }
            for j in i+1..b.len() {
                // for k in j+1..n {
                    let (mut q, mut qc) = b[j];
                    remaining += qc;
            }
        }
        sayln!("{}", ans);
    }
}
