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
        let n = scan!(u32);
        let mut a = (0..n).map(|_| scan!(i64)).vec();
        if a.iter().any(|&e| e == 0) {
            sayln!("-1");
            continue;
        }
        let indices = (0..40)
            .map(|i| 1i64 << i)
            .map(|e| {
                a.iter()
                    .cloned()
                    .enumerate()
                    .filter(|(_, b)| b & e > 0)
                    .map(|(i, _)| i)
                    .vec()
            })
            .vec();

        let mut not_yet = HashSet::new();
        let mut sum = 0;
        let mut mask = 0i64;
        let mut masks = vec![];
        let mut have = HashSet::<usize>::new();
        for (i, indices) in indices.into_iter().enumerate() {
            let l = 1i64 << i;
            if indices.len() == 0 {
                masks.push(0);
                continue;
            }
            if indices.len() == 1 {
                masks.push(0);
                if have.contains(&indices[0]) {
                    continue;
                } else {
                    not_yet.insert(indices[0]);
                    continue;
                }
            }
            let mut curr_mask = 0;
            let mut connected = false;
            let mut c = indices
                .into_iter()
                .filter(|e| {
                    let b = !have.contains(e);
                    connected |= !b;
                    curr_mask |= *e as i64;
                    have.insert(*e);
                    b
                })
                .count() as i64;
            // let mut one = 0;
            // for i in 0..i {
            //     one |= 1 << i;
            // }
            // curr_mask &= one;
            // if mask & curr_mask == 0 {

            // }
            // masks.push(curr_mask);
            if c > 1 {
                c -= 1;
            }
            if !connected && mask & curr_mask > 0 {
                c += 1;
            }
            mask |= curr_mask;
            // if c == 1 {
            //     c += 1;
            // }
            sum += l * c;
        }
        if not_yet.into_iter().any(|e| !have.contains(&e)) {
            sayln!("-1");
            continue;
        }
        sayln!("{}", sum);
    }
}
