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
        let n = scan!();
        let a = (0..n).map(|_| scan!(u32)).vec();

        let mut front = a.iter().copied().take(n / 2).rev().vec();
        let mut back = a.iter().copied().rev().take(n / 2).rev().vec();

        let mut mid = if n % 2 == 0 { None } else { Some(a[n / 2]) };

        let mut count = 0;
        // dbg!(());
        loop {
            // dbg!(&front, &back, mid);
            match (front.pop(), back.pop()) {
                (Some(i), Some(j)) => {
                    // dbg!((i, j));
                    match i.cmp(&j) {
                        std::cmp::Ordering::Less => {
                            back.push(j - i);
                            count += 1;
                        }
                        std::cmp::Ordering::Greater => {
                            front.push(i - j);
                            count += 1;
                        }
                        std::cmp::Ordering::Equal => (),
                    }
                }
                (None, Some(j)) => {
                    match mid {
                        Some(i) => {
                            // dbg!((i, j));
                            match i.cmp(&j) {
                                std::cmp::Ordering::Less => {
                                    // front.push(j-i);
                                    back.push(j - i);
                                    count += 1;
                                }
                                std::cmp::Ordering::Equal => (),
                                std::cmp::Ordering::Greater => {
                                    // back.push(i-j);
                                    front.push(i - j);
                                    count += 1;
                                }
                            }
                            mid = None;
                        }
                        None => {
                            if back.len() > 0 {
                                back.push(j);
                                let n = back.len();
                                mid = if n % 2 == 0 { None } else { Some(back[n / 2]) };
                                front = back.iter().copied().take(n / 2).rev().vec();
                                back = back.iter().copied().rev().take(n / 2).rev().vec();
                            }
                        }
                    }
                }
                (Some(i), None) => {
                    match mid {
                        Some(j) => {
                            // dbg!((i, j));
                            match i.cmp(&j) {
                                std::cmp::Ordering::Less => {
                                    // front.push(j-i);
                                    back.push(j - i);
                                    count += 1;
                                }
                                std::cmp::Ordering::Equal => (),
                                std::cmp::Ordering::Greater => {
                                    front.push(i - j);
                                    // back.push(i-j);
                                    count += 1;
                                }
                            }
                            mid = None;
                        }
                        None => {
                            if front.len() > 0 {
                                front.push(i);
                                let n = front.len();
                                mid = if n % 2 == 0 { None } else { Some(front[n / 2]) };
                                back = front.iter().rev().copied().rev().take(n / 2).rev().vec();
                                front = front.iter().rev().copied().take(n / 2).rev().vec();
                            }
                        }
                    }
                }
                (None, None) => break,
            }
        }
        sayln!("{}", count);
    }
}
