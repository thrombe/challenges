#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::io::{BufWriter, stdin, stdout, Write};
 
#[derive(Default)]
struct Scanner {
    buffer: Vec<String>
}
impl Scanner {
    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut input = String::new();
            stdin().read_line(&mut input).expect("Failed read");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }
}

// BAD: failed
// it is incorrect as its possible that there will be cases where multiplying by the max
// digit leads to bad numbers like 11111111 or 121212221 and multiplying by the not max
// digit leads to numbers like 1212129, so traversing of whole tree smartly is needed
// so only traverse those subtrees, which can lead to a answer less then the min
// answer found yet
fn main() {
    let mut s = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let n = s.next::<usize>();
    let mut x = s.next::<usize>();
    if digits(x).iter().all(|&e| e==1) && n > digits(x).len() {
        writeln!(out, "-1").unwrap();
        return;
    }
    
    // dbg!(xs);
    let mut c = 0;
    'loup: loop {
        let mut xs = digits(x);
        xs.sort();
        xs.dedup();
        for &d in xs.iter().rev() {
            let new_x = d*x;
            // dbg!(d, new_x);
            let ds = digits(new_x);
            if ds.len() < n {
                x = new_x;
                c += 1;
                dbg!(x, d, ds.len(), c);
                continue 'loup;
            } else if ds.len() == n {
                x = new_x;
                c += 1;
                dbg!(x, d, ds.len(), c);
                writeln!(out, "{c}").unwrap();
                return;
            }
        }
        writeln!(out, "-1").unwrap();
        return;
    }
    
}

fn digits(x: usize) -> Vec<usize> {
    let mut ex = x;
    let xs = (0..).map(|_| {
        let x = ex%10;
        if ex > 0 {
            ex = ex/10;
            Some(x)
        } else {
            None
        }
    }).take_while(Option::is_some).map(Option::unwrap).collect::<Vec<_>>();
    xs
}
