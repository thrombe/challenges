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

// fn main2() {
//     let n = scan!(usize);
//     let a = (0..n).into_iter().map(|_| scan!(u64) == 1).collect::<Vec<_>>();
//     let edges = (0..n-1)
//     .into_iter()
//     .map(|_| (scan!(usize)-1, scan!(usize)-1))
//     .map(|(a, b)| (a.min(b), a.max(b)))
//     .collect::<Vec<_>>();
//     let e_neighbours = {
//         let mut nei = vec![0;n];
//         for i in 0..n {
//             nei[i] += if a[i] {1} else {0};
//         }
//         for &(i, j) in &edges {
//             nei[j] += if a[i] {1} else {0};
//             nei[i] += if a[j] {1} else {0};
//         }
//         // dbg!(&nei);
//         nei
//     };
//     // let all = {
//     //     let mut all = vec![vec![];n];
//     //     for &(a, b) in &edges {
//     //         all[a].push(b);
//     //         // all[b].push(a);
//     //     }
//     //     // dbg!(&all);
//     //     all
//     // };
//     let parents = {
//         let mut pths = vec![0;n]; // parents
//         for &(a, b) in &edges {
//             pths[b] = a;
//         }
//         // for i in 1..n {
//         //     let mut a = Some(i);
//         //     while let Some(a) = a {
//         //         pths[i].push(pths[])
//         //     }

//         // //     for &(a, b) in &edges {
//         // //         if
//         // //     }
//         // }
//         // dbg!(&pths);
//         pths
//     };
//     let q = scan!(u32);
//     for _ in 0..q {
//         let to = scan!(usize)-1;
//         let min_nodes = scan!(usize);
//         let min_e = scan!(usize);
//         // dbg!(to, min_e, min_nodes);

//         // let mut path = vec![0];
//         // get_path(&mut path, to, &all, 0);
//         // dbg!(&path);
//         let path = {
//             let mut a = Some(parents[to]);
//             let mut pth = vec![to];
//             while let Some(b) = a {
//                pth.push(b);
//                if b != 0 {
//                     a = Some(parents[b]);
//                } else {
//                     a = None;
//                }
//             }
//             pth
//         };
//         // dbg!(&path);

//         let mut acc = path.iter().cloned()
//         .map(|i| e_neighbours[i])
//         .filter(|&e| e>=1)
//         .collect::<Vec<_>>();
//         if acc.len() < min_nodes {
//             sayln!("-1");
//         } else {
//             acc.sort_by(|a, b| b.cmp(a));
//             // let mut ans = min_e / acc[min_nodes-1] + if min_e % acc[min_nodes-1] ==0 {0} else {1};
//             // dbg!(&acc, acc[min_nodes-1], min_e, ans, "");
//             // sayln!("{}", ans);

//             // let m = acc[min_nodes-1];
//             // let mut e = 0;
//             // let mut sec = 0;
//             // for i in 1.. {
//             //     e += m*i;
//             //     sec += 1;
//             //     if e >= min_e {
//             //         break;
//             //     }
//             // }

//             let min_ef = min_e as f64;
//             let m = acc[min_nodes-1] as f64;
//             let k = 2.0*min_ef/m;
//             let sk = k.sqrt();
//             let np1 = sk;
//             let n = np1.round() as usize;
//             if (n-1)*(n)/2*acc[min_nodes-1] >= min_e {
//                 sayln!("{}", n-1);
//             } else if (n+1)*(n)/2*acc[min_nodes-1] >= min_e {
//                 sayln!("{}", n);
//             } else if (n+1)*(n+2)/2*acc[min_nodes-1] >= min_e {
//                 sayln!("{}", n+1);
//             } else {
//                 panic!();
//             }
//         }
//     }
// }

fn main() {
    let n = scan!(usize);
    let a = (0..n)
        .into_iter()
        .map(|_| scan!(u64) == 1)
        .collect::<Vec<_>>();
    let edges = (0..n - 1)
        .into_iter()
        .map(|_| (scan!(usize) - 1, scan!(usize) - 1))
        // .map(|(a, b)| (a.min(b), a.max(b)))
        .collect::<Vec<_>>();
    let e_neighbours = {
        let mut nei = vec![0; n];
        for i in 0..n {
            nei[i] += if a[i] { 1 } else { 0 };
        }
        for &(i, j) in &edges {
            nei[j] += if a[i] { 1 } else { 0 };
            nei[i] += if a[j] { 1 } else { 0 };
        }
        // dbg!(&nei);
        nei
    };
    let path = {
        let mut all = vec![vec![]; n];
        for &(a, b) in &edges {
            all[a].push(b);
            all[b].push(a);
        }
        // dbg!(&all);
        all
    };
    let path_to = {
        let mut pths = vec![vec![]; n];
        let mut stack = vec![0];
        let mut visited = std::collections::HashSet::new();
        visited.insert(0);

        fn re(
            l: &mut Vec<usize>,
            all: &[Vec<usize>],
            curr: usize,
            pths: &mut Vec<Vec<usize>>,
            v: &mut std::collections::HashSet<usize>,
        ) {
            // dbg!(&l);
            for &i in &all[curr] {
                // dbg!(curr, i);
                if v.contains(&i) {
                    continue;
                }
                l.push(i);
                pths[i] = l.clone();
                v.insert(i);
                re(l, all, i, pths, v);
                let _ = l.pop();
            }
        }
        re(&mut stack, &path, 0, &mut pths, &mut visited);
        // dbg!(&pths);
        pths
    };
    let q = scan!(u32);
    for _ in 0..q {
        let to = scan!(usize) - 1;
        let min_nodes = scan!(usize);
        let min_e = scan!(usize);
        // dbg!(to, min_e, min_nodes);

        let path = &path_to[to];

        let mut acc = path
            .iter()
            .cloned()
            .map(|i| e_neighbours[i])
            .filter(|&e| e >= 1)
            .collect::<Vec<_>>();
        if acc.len() < min_nodes {
            sayln!("-1");
        } else {
            acc.sort_by(|a, b| b.cmp(a));
            // let mut ans = min_e / acc[min_nodes-1] + if min_e % acc[min_nodes-1] ==0 {0} else {1};
            // dbg!(&acc, acc[min_nodes-1], min_e, ans, "");
            // sayln!("{}", ans);

            // let m = acc[min_nodes-1];
            // let mut e = 0;
            // let mut sec = 0;
            // for i in 1.. {
            //     e += m*i;
            //     sec += 1;
            //     if e >= min_e {
            //         break;
            //     }
            // }

            let min_ef = min_e as f64;
            let m = acc[min_nodes - 1] as f64;
            let k = 2.0 * min_ef / m;
            let sk = k.sqrt();
            let np1 = sk;
            let n = np1.round() as usize;
            if n >= 2 && (n - 2) * (n - 1) / 2 * acc[min_nodes - 1] >= min_e {
                panic!();
            } else if (n - 1) * (n) / 2 * acc[min_nodes - 1] >= min_e {
                sayln!("{}", n - 1);
            } else if (n) * (n + 1) / 2 * acc[min_nodes - 1] >= min_e {
                sayln!("{}", n);
            } else if (n + 1) * (n + 2) / 2 * acc[min_nodes - 1] >= min_e {
                sayln!("{}", n + 1);
            } else {
                panic!();
            }
        }
    }
}

// fn get_path(l: &mut Vec<usize>, to: usize, all: &[Vec<usize>], curr: usize) -> bool {
//     // dbg!(&l);
//     for &i in &all[curr] {
//         // dbg!(curr, i);
//         l.push(i);
//         if i == to {return true}
//         if get_path(l, to, all, i) {
//             return true;
//         } else {
//             let _ = l.pop();
//         }
//     }
//     false
// }
