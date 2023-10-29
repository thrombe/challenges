#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::io::{stdin, stdout, BufWriter, Write};
use std::prelude::v1::*;
use std::str::{FromStr, SplitWhitespace};

struct Scanner {
    buffer: SplitWhitespace<'static>,
    leaked: *mut str,
}
impl Scanner {
    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.next() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut input = unsafe { String::from(Box::from_raw(self.leaked)) };
            input.clear();
            stdin().read_line(&mut input).expect("Failed read");
            self.leaked = Box::into_raw(input.into_boxed_str());
            let v = unsafe { &*self.leaked };
            self.buffer = v.split_whitespace();
            // let _ = Box::leak(olb);
        }
    }
}
impl Default for Scanner {
    fn default() -> Self {
        let input = String::new();
        let leaked = Box::into_raw(input.into_boxed_str());
        let v = unsafe { &*leaked };
        let buffer = v.split_whitespace();
        Self { buffer, leaked }
    }
}
impl Drop for Scanner {
    fn drop(&mut self) {
        let unleak = unsafe { Box::from_raw(self.leaked) };
    }
}

// // TODO: some kinda safe abstraction for this
// // leak and unleak the string to have mutable access to it. while compiler thinks it
// fn new_input(input: &mut String) {
//     loop {
//         input.clear();
//         stdin().read_line(input).unwrap();
//         if input.trim().len() > 0 {
//             break;
//         }
//     }
// }
// fn get_next<'a, T: FromStr>(iter: &mut Option<SplitWhitespace<'a>>) -> Option<T> {
//     iter.as_mut().map(|mut iter| iter.next().map(|e| e.parse().ok().unwrap())).flatten()
// }
// fn next<'a, 'b: 'a, T: FromStr>(iter: &mut Option<SplitWhitespace<'a>>, input: &'b mut String) -> T {
//     if let Some(i) = iter {
//         if let Some(e) = get_next(iter) {
//             e
//         } else {
//             new_input(input);
//             *iter = Some(input.split_whitespace());
//             get_next(iter).unwrap()
//         }
//     } else {
//         new_input(input);
//         *iter = Some(input.split_whitespace());
//         get_next(iter).unwrap()
//     }
// }

fn main() {
    let mut scanner = Scanner::default();
    let writer = &mut BufWriter::new(stdout());
    macro_rules! sayln {
        ($($arg:tt)*) => (
            writeln!(writer, $($arg)*).unwrap();
        );
    }
    macro_rules! say {
        ($($arg:tt)*) => (
            write!(writer, $($arg)*).unwrap();
        );
    }
    macro_rules! scan {
        ($i:ident) => {
            scanner.next::<$i>()
        };
        () => {
            scanner.next()
        };
    }

    (0..scan!(u32)).for_each(|_| {
        dbg!(scan!(u32));
    });
}
