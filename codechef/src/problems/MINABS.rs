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
    'outer: for _ in 0..scan!(u32) {
        let n = scan!();
        let a = scan!(&str).chars().map(|e| e as i32 - 'a' as i32).vec();
        let b = scan!(&str).chars().map(|e| e as i32 - 'a' as i32).vec();

        let rs = (0..n).map(|i| (52 + b[i] - a[i]) % 26).vec();
        let ls = (0..n).map(|i| (52 + a[i] - b[i]) % 26).map(|e| -e).vec();
        // dbg!(&rs, &ls);

        // let mut rxl = vec![vec![i32::MAX ; n] ; n];
        // rxl[0][0] = 0;
        // for i in 1..n {
        //     rxl[i][0] = rxl[i-1][0] + rs[i];
        //     rxl[0][i] = rxl[0][i-1] + ls[i];
        //     rxl[i][i] = 0;
        // }
        // for i in 1..n {
        //     for j in 1..n {
        //     }
        // }

        // for line in rxl {
        //     for i in line {
        //         say!("{i:010} ");
        //     }
        //     sayln!();
        // }
        // sayln!();

        // let mut rights = vec![ls.iter().sum::<i32>()];
        // for i in 1..n {
        //     (0..n).map(|j| rights[i-1])
        // }

        let mut val = rs.iter().sum::<i32>();
        let mut min = val;
        for _ in rs.iter().copied().filter(|&e| e != 0) {
            if val == 0 {
                sayln!("0");
                continue 'outer;
            } else if val > 26 {
                val -= 26;
            } else {
                sayln!("{}", i32::min(val, (val - 26).abs()));
                continue 'outer;
            }
        }
        sayln!("{}", val);
    }
}
