#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::fmt::format;
use std::io::{BufWriter, stdin, stdout, Write, Stdout};
use std::str::{SplitWhitespace, FromStr};
use std::cell::UnsafeCell;
use std::prelude::v1::*;
 
struct Scanner {
    buffer: SplitWhitespace<'static>,
    leaked: *mut str,
}
impl Scanner {
    fn next<T: std::str::FromStr>(&mut self) -> T {
        self.next_raw().parse().ok().expect("Failed parse")
    }
    fn next_raw<'a>(&'a mut self) -> &'a str {
        loop {
            if let Some(token) = self.buffer.next() { return token; }
            let mut input = unsafe { String::from(Box::from_raw(self.leaked)) };
            input.clear();
            stdin().read_line(&mut input).expect("Failed read");
            self.leaked = Box::into_raw(input.into_boxed_str());
            let v = unsafe { &*self.leaked };
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
    (&str) => { unsafe { SCANNER.with(|mut e| (*e.get()).next_raw()) } };
    () => { unsafe { SCANNER.with(|mut e| (*e.get()).next()) } };
}

fn main() {
    let l = scan!(u64);
    let r = scan!(u64);
    let ls = (0..).map(|i| l/10u64.pow(i)).take_while(|&i| i != 0).map(|e| e%10).collect::<Vec<_>>();
    let lr = (0..).map(|i| r/10u64.pow(i)).take_while(|&i| i != 0).map(|e| e%10).collect::<Vec<_>>();
    // dbg!(ls, lr);

    let mut cnt = 0;
    if ls[0] == ls[ls.len()-1] {
        cnt += 1;
    }

    // ?

    if ls[0] < 9 && ls.len() > 2 {
        cnt += (9-ls[0] - 1)*10u64.pow((ls.len()-2) as u32);
    }
    for i in (ls.len().min(2)-2)..lr.len()-2 {
        cnt += 9*10u64.pow(i as u32)
    }

    cnt += (ls[0] - 1)*10u64.pow((ls.len()-2) as u32);
    

    sayln!("{}", cnt);
}