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

#[derive(Copy, Clone, Debug)]
struct A {
    state: bool,
    prio: usize,
}

fn main() {
    for _ in 0..scan!(u32) {
        let s = scan!(&str).chars().map(|c| c == '1').vec();
        let n = s.len();
        let prio = (0..n).map(|i| (n - i) * (i + 1)).vec();
        let mut vals = (0..n)
            .map(|i| A {
                state: s[i],
                prio: prio[i],
            })
            .vec();
        let mut vals = {
            let mut v = vec![];
            let mut a = A {
                state: vals[0].state,
                prio: 0,
            };
            for e in vals {
                if a.state == e.state {
                    a.prio += e.prio;
                } else {
                    v.push(a);
                    a = e;
                }
            }
            v.push(a);
            // dbg!(&v);
            v
        };
        // vals.sort_by(|a, b| a.prio.cmp(&b.prio));

        // let mut ans = vals.iter().copied().fold(0, |a, e| a + if e.state {e.prio} else {0});
        // let mut max = ans;
        let mut ans = 0;
        if vals[0].state {
            let a = vals.remove(0);
            ans += a.prio;
        }
        if let Some(e) = vals.last() {
            if e.state {
                let a = vals.pop();
                ans += a
                    .unwrap_or(A {
                        state: true,
                        prio: 0,
                    })
                    .prio;
            }
        }

        // dbg!(&vals);
        // now just have to find such a subarray in "vals", that when inversed gives max increase in answer
        //? https://www.geeksforgeeks.org/largest-sum-contiguous-subarray/
        let mut max_so_far = 0;
        let mut max_ending_here = max_so_far;
        let mut start = 0;
        let mut end = 0;
        let mut s = 0;

        for i in 0..vals.len() {
            max_ending_here += vals[i].prio as i32 * if !vals[i].state { 1 } else { -1 };
            if max_so_far < max_ending_here {
                max_so_far = max_ending_here;
                start = s;
                end = i + 1;
            }
            if max_ending_here < 0 {
                max_ending_here = 0;
                s = i + 1;
            }
        }
        // dbg!(start, end);

        sayln!(
            "{}",
            vals.iter().enumerate().fold(ans, |a, e| a + match (
                e.1.state,
                (start..end).contains(&e.0)
            ) {
                // inside flipped subarray
                (true, true) => 0,
                (false, true) => e.1.prio,

                // outside flipped subarray
                (true, false) => e.1.prio,
                (false, false) => 0,
            })
        );
    }
}
