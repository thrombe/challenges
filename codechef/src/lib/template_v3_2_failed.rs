#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::io::{ BufWriter, stdin, stdout, Write, Stdout };
use std::iter::Map;
use std::str::{ SplitWhitespace, FromStr };
use std::cell::UnsafeCell;

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
struct It<P: Iterator<Item=T>, T> {inner: P}
impl<P: Iterator<Item=T>, T> Iterator for It<P, T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}
trait Brom<I, T>
    where I: Iterator<Item = T>,
{
    fn brom(_: I) -> It<I, T>;
}
impl<P, T> Brom<P, T> for It<P, T>
    where
    P: Iterator<Item = T>,
{
    fn brom(inner: P) -> It<P, T> {
        It {inner}
    }
}

// impl<P, T, I> From<I> for It<P, T>
//     where
//     I: Iterator<Item=T>,
// {

// }
trait Scn<T>: Iterator<Item=T> where Self: Sized {
    fn scan<F: FnMut(T) -> O, O>(self) -> Map<Self, F>;
}
impl<T, I, K> Scn<T> for K
where
I: Iterator<Item = T>,
K: Brom<I, T>,
{

}
// impl<T, P> Scn<T> for It<P, T>
//     where
//     P: Iterator<Item=T>,
//     // F: FnMut(T) -> O,
// {
//     fn scan<F: FnMut(T) -> O, O>(self) -> Map<Self, F> {
//         self.map(|_| scan!())
//     }
// }

fn main() {
    
}