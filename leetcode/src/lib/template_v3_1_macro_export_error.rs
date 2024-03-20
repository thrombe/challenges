#![allow(unused_imports, unused_macros, unused_variables, unused_mut)]





fn main() {
    
}























































pub use std::io::{ BufWriter, stdin, stdout, Write, Stdout };
pub use std::str::{ SplitWhitespace, FromStr };
pub use std::cell::UnsafeCell;
use std::collections::{ HashMap, HashSet, BinaryHeap };

pub struct Scanner {
    buffer: SplitWhitespace<'static>,
    leaked: *mut str,
}
impl Scanner {
    pub fn next<T: std::str::FromStr>(&mut self) -> T {
        unsafe { self.next_raw() }.parse().ok().expect("Failed parse")
    }
    pub unsafe fn next_raw<'a>(&'a mut self) -> &'a str {
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
pub trait Vek<T>: Iterator<Item=T> {
    fn vec(self) -> Vec<T>;
}
impl<T, P: Iterator<Item=T>> Vek<T> for P {
    fn vec(self) -> Vec<T> { self.collect() }
}

#[macro_export]
macro_rules! _sayln {
    ($($arg:tt)*) => { writeln!(unsafe { WRITER.with(|e| &mut (*e.get())) }, $($arg)*).unwrap(); };
}
#[macro_export]
macro_rules! _say {
    ($($arg:tt)*) => { write!(unsafe { WRITER.with(|e| &mut (*e.get())) }, $($arg)*).unwrap(); };
}
#[macro_export]
macro_rules! _scan {
    ($i:ident) => { unsafe { SCANNER.with(|mut e| (*e.get()).next::<$i>()) } };
    () => { unsafe { SCANNER.with(|mut e| (*e.get()).next()) } };
    
    // using SCANNER after calling this might invalidate this refrence returned by this call. unsafe!!. but this is slightly faster when it is safe to use this
    (&str) => { unsafe { SCANNER.with(|mut e| (*e.get()).next_raw()) } };
}

pub use _sayln as sayln;
pub use _say as say;
pub use _scan as scan;

thread_local! {
    static SCANNER: UnsafeCell<Scanner> = Scanner::default().into();
    static WRITER: UnsafeCell<BufWriter<Stdout>> = BufWriter::new(stdout()).into();
}
