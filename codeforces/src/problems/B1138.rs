
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

/*
(ci, ai)
p, q, r, s -> num of (0, 0), (1, 0), (0, 1), (1, 1) in first performance
nt -> total num of t

p+q+r+s = n/2
q+s = nr-r + ns-s   =>    r+q+2s = nr+ns

=> p-s = n/2 - nr - ns
*/
fn main() {
    let mut s = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let n = s.next::<usize>();
    let c_s = s.next::<String>();
    let a_s = s.next::<String>();

    let c = c_s.chars().map(|c| c as u64 - '0' as u64 == 1);
    let a = a_s.chars().map(|a| a as u64 - '0' as u64 == 1);

    let z = c.clone().zip(a.clone());
    let np = z.clone().filter(|(c, a)| !*a && !*c).count();
    let nq = z.clone().filter(|(c, a)| !*a && *c).count();
    let nr = z.clone().filter(|(c, a)| *a && !*c).count();
    let ns = z.clone().filter(|(c, a)| *a && *c).count();
    // dbg!(np, nq, nr, ns);

    for s in 0..ns+1 {
        let p = (n/2 + s) as i64 - (nr + ns) as i64;
        for r in 0..nr+1 {
            let q = (nr+ns) as i64-(2*s+r) as i64;
            if p >= 0 && p <= np as i64 && q >= 0 && q <= nq as i64 {
                let (mut p, mut q, mut r, mut s) = (p as usize, q as usize, r, s);
                // dbg!(p, q, r, s);
                for (i, (c, a)) in z.clone().enumerate() {
                    if !c && !a {
                        if p != 0 {
                            p -= 1;
                        } else {
                            continue;
                        }
                    } else if c && !a {
                        if q != 0 {
                            q -= 1;
                        } else {
                            continue;
                        }
                    } else if !c && a {
                        if r != 0 {
                            r -= 1;
                        } else {
                            continue;
                        }
                    } else {
                        if s != 0 {
                            s -= 1;
                        } else {
                            continue;
                        }
                    }
                    write!(out, "{} ", i+1).unwrap();
                }
                // dbg!(p, q, r, s);
                return;
            }
        }
    }
    writeln!(out, "{}", -1).unwrap();
}