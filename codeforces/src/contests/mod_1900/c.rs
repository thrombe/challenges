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

// for _ in range(int(input())):
//     n = int(input())
//     s = list(input().lower())

//     children = []
//     parent = [(-1, 'h')]*n
//     leaves = []
//     dp = [float('inf')]*n
//     for i in range(n):
//         el, ar = map(int, input().split())
//         el -= 1
//         ar -= 1

//         children.append((el, ar))
//         if max(el, ar) == -1:
//             leaves.append(i)
//             dp[i] = 0
//         else:
//             if el > -1:
//                 parent[el] = (i, 'l')
//             if ar > -1:
//                 parent[ar] = (i, 'r')

//     # print(children)
//     # print(leaves)
//     # print(parent)

//     traversing = leaves.copy()
//     next = set()
//     while len(traversing) > 0:
//         if dp[0] == 0:
//             break
//         # print(traversing, s)
//         # print(next, "before loop")
//         for i in traversing:
//             p, d = parent[i]
//             if p == -1:
//                 continue
//             next.add(p)
//             if d == s[p]:
//                 dp[p] = min(dp[p], dp[i])
//             else:
//                 dp[p] = min(dp[p], dp[i] + 1)
//         # print(next, "after loop")

//         traversing = next
//         next = set()
//         # import time
//         # time.sleep(2)

//     # print(dp)
//     print(dp[0])
fn main() {
    for _ in 0..scan!(u32) {
        let n = scan!(usize);
        let s: Vec<char> = scan!(&str).chars().collect();

        // let mut children = vec![];
        let mut parent = vec![(0, 'H');n];
        let mut leaves = vec![];
        let mut dp = vec![u64::MAX;n];
        for i in 0..n {
            let el = scan!(usize);
            let ar = scan!(usize);

            // children.append(el, ar);
            if (0, 0) == (el, ar) {
                leaves.push(i + 1);
                dp[i] = 0;
            } else {
                if el > 0 {
                    parent[el - 1] = (i + 1, 'L');
                }
                if ar > 0 {
                    parent[ar - 1] = (i + 1, 'R');
                }
            }
        }
        // dbg!(&parent, &leaves);

        let mut traversing: HashSet<_> = leaves.iter().copied().collect();
        let mut next: HashSet<usize> = Default::default();
        while !traversing.is_empty() {
            // dbg!(&traversing);
            for i in traversing.iter().copied() {
                let (p, d) = parent[i - 1];
                if p == 0 {
                    continue;
                }

                next.insert(p);
                if d == s[p - 1] {
                    dp[p - 1] = dp[p - 1].min(dp[i - 1]);
                } else {
                    dp[p - 1] = dp[p - 1].min(dp[i - 1] + 1);
                }
            }
            traversing = next;
            next = Default::default();
        }
        // dbg!(&dp);

        sayln!("{}", dp[0]);
    }
}
