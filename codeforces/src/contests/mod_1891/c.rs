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
    unsafe fn next_raw(&mut self) -> &str {
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

fn main() {
    for _ in 0..scan!(u32) {
        let n = scan!(usize);
        let mut a = (0..n).map(|_| scan!()).collect::<Vec<usize>>();
        a.sort();

        if n == 2 {
            sayln!("{}", a[0] + (a[1]-a[0])/2 + 1 + (a[1] - a[0])%2);
            continue;
        }

        let mut ps = vec![0usize;a.len()];

        let mut ops = 0;
        // dbg!(&a);
        while a[n-1] > 0 {
            ps[0] = a[0];
            for i in 1..a.len() {
                ps[i] = ps[i-1]+a[i];
            }

            match ps.binary_search(&a[n-1]) {
                Ok(i) | Err(i)=> {
                    // dbg!(i, ops, &a);
                    if i == n-1 {
                        if a[i] == 1 {
                            ops += 1;
                            a[i] = 0;
                        } else {
                            ops += ps[i-1] + (a[n-1] - a[n-2])/2 + 1 + (a[n-1] - a[n-2])%2;
                            a[n-1] = 0;
                        }
                        continue;
                    }
                    (0..i+1).for_each(|i| a[i] = 0);
                    a[n-1] -= ps[i];
                    if i == n-1 {
                        // dbg!("lol");
                        ops += ps[i] + (a[n-1])/2 + 1 + (a[n-1])%2;
                        a[n-1] = 0;
                    } else {
                        ops += ps[i]+1;
                    }
                    let o = a.pop().unwrap();
                    match a.binary_search(&o) {
                        Ok(j) | Err(j) => {
                            a.insert(j, o);
                        }
                    }
                },
            }
        }
        // dbg!(ops);
        sayln!("{}", ops);
    }
}
