#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::cell::UnsafeCell;
use std::io::{stdin, stdout, BufWriter, Stdout, Write};
use std::str::{FromStr, SplitWhitespace};

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

/*
for _ in range(int(input())):
    n = int(input())
    m = list(map(int, input().split()))
    sorted_m = m[:]
    sorted_m.sort(key = lambda x: -x)
    cached_answers = {}
    q = int(input())
    for _ in range(q):
        p, k = map(int, input().split())
        if cached_answers.get(p, None) is None:
            ms = filter(lambda x: x%p == 0, sorted_m)
            s = [0]
            for i in m:
                if i%p == 0:
                    s.append(next(ms)+s[len(s)-1])
                else:
                    s.append(i + s[len(s)-1])
            cached_answers[p] = s
        print(cached_answers[p][k])
*/

use std::collections::{HashMap, HashSet};

fn main() {
    // ? what is this code lmao, prime detector is just number%p == 0, idk what i was thinking. atleast this tech can help in other probs maybe
    // dbg!(detecc(100));
    // let t = std::time::SystemTime::now();
    let detector = detecc(100000); // 0.416 sec
                                   // dbg!(std::time::SystemTime::now().duration_since(t).unwrap().as_secs_f64());

    for _ in 0..scan!(u32) {
        let n = scan!(u32);
        let m = (0..n).map(|_| scan!(u32)).collect::<Vec<_>>();
        let mut sorted_m = m.clone();
        sorted_m.sort_by(|a, b| b.cmp(a));
        for _ in 0..scan!(u32) {
            let p = scan!(u32);
            let k = scan!(u32);
            let fav_detector = &detector[&p];
            let mut ms = sorted_m.iter().filter(|&e| fav_detector.contains(e));
            let mut s = 0;
            for i in &m[..k as usize] {
                if fav_detector.contains(i) {
                    s += ms.next().unwrap();
                } else {
                    s += i;
                }
            }
            sayln!("{}", s);
        }
    }
}

fn detecc(n: u32) -> HashMap<u32, HashSet<u32>> {
    let n = (n + 1) as usize;
    let mut not_prime = vec![false; n as usize];
    let mut detector = HashMap::new();
    for i in 2..n {
        if not_prime[i] {
            continue;
        }
        let mut nprimes = HashSet::new();
        nprimes.insert(i as u32);
        for f in (0..).map(|e| i * 2 + e * i).take_while(|&e| e < n) {
            not_prime[f] = true;
            nprimes.insert(f as u32);
        }
        detector.insert(i as u32, nprimes);
    }
    detector.insert(1, (0..n as u32).collect());
    detector
}
