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
        let s = scan!(&str).chars().rev().map(|c| c == '1').vec();

        let p = s
            .iter()
            .copied()
            .enumerate()
            .map(|(i, e)| if (i + 1) % 2 == 1 { 1 } else { 2 })
            .sum::<u32>()
            % 3;
        // dbg!(p);

        match p {
            0 => {
                sayln!("0");
            }
            // 1 | 2 => {
            //     let c1 = s.iter().copied().enumerate()
            //     .filter(|&(i, e)| (i+1)%2 == if p ==1 {0} else {1})
            //     .filter(|&(i, e)| e)
            //     .any(|(i, e)| !*s.get(i-1).unwrap_or(&true) || !*s.get(i+1).unwrap_or(&true));

            //     let c2 = s.iter().copied().all(|e| e) || s.iter().copied().all(|e| !e);

            //     let c3 = if s.iter().filter(|&&e| e ).count() == 1 {
            //         if let Some(i) = s.iter().position(|e| *e) {
            //             if i%2 == 0 {
            //                 if p ==1 {false} else {true}
            //             } else {
            //                 if p ==1 {0} else {1}
            //             }
            //         } else {
            //             if p ==1 {0} else {1}
            //         }
            //     } else {
            //         false
            //     };

            //     if s[0] {
            //         s.iter().cloned().
            //     } else {

            //     }
            // }
            1 | 2 => {
                // if 1
                // find an immediate decrement
                // find 2 immediate increments
                // find a 3 swap decrement

                // if 2
                // find an immediate increment
                // find 2 immediate decrements
                // find a 3 swap increment

                let c1 = s
                    .iter()
                    .copied()
                    .enumerate()
                    .filter(|&(i, e)| (i + 1) % 2 == if p == 1 { 0 } else { 1 })
                    .filter(|&(i, e)| e)
                    .any(|(i, e)| {
                        !*s.get(i - 1).unwrap_or(&true) || !*s.get(i + 1).unwrap_or(&true)
                    });

                let mut right_zero_index = None;
                let mut count = 0;
                s.iter()
                    .copied()
                    .enumerate()
                    .filter(|&(i, e)| (i + 1) % 2 == if p == 1 { 1 } else { 0 })
                    .filter(|&(i, e)| e)
                    .for_each(|(i, e)| {
                        if count >= 2 {
                            return;
                        } else {
                            match (
                                !*s.get(i - 1).unwrap_or(&true),
                                !*s.get(i + 1).unwrap_or(&true),
                            ) {
                                (true, true) => {
                                    count += 1;
                                    right_zero_index = Some(i + 1);
                                }
                                (true, false) => {
                                    if right_zero_index != Some(i - 1) {
                                        count += 1;
                                    }
                                    right_zero_index = None;
                                }
                                (false, true) => {
                                    count += 1;
                                    right_zero_index = Some(i + 1);
                                }
                                (false, false) => {
                                    right_zero_index = None;
                                }
                            }
                        }
                    });
                let c2 = count >= 2;

                let c3 = s
                    .iter()
                    .copied()
                    .enumerate()
                    .filter(|&(i, e)| (i + 1) % 2 == if p == 1 { 0 } else { 1 })
                    .filter(|&(i, e)| e)
                    .any(|(i, e)| {
                        if (!*s.get(i - 3).unwrap_or(&true)
                            && !*s.get(i - 2).unwrap_or(&true)
                            && *s.get(i - 1).unwrap_or(&false))
                            || (*s.get(i + 1).unwrap_or(&false)
                                && !*s.get(i + 2).unwrap_or(&true)
                                && !*s.get(i + 3).unwrap_or(&true))
                        {
                            true
                        } else {
                            false
                        }
                    });

                if c1 {
                    sayln!("1");
                } else if c2 {
                    sayln!("2");
                } else if c3 {
                    sayln!("3");
                } else {
                    sayln!("-1");
                }
            }
            _ => unreachable!(),
        }
    }
}
