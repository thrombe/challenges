
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


fn main() {
    let mut s = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let n = s.next::<i64>();
    let m = s.next::<i64>() + 2;
    let mut floors = vec![];

    for _ in 0..n {
        let mut coords = (-1, -1);
        for (i, c) in s.next::<String>().chars().enumerate() {
            if c == '1' {
                if coords.0 == -1 {
                    coords.0 = i as i64;
                    coords.1 = i as i64;
                } else {
                    coords.1 = i as i64;
                }
            }
        }
        floors.push(coords);
    }

    writeln!(out, "{}", rec(m, &floors, (0, n-1))).unwrap();
}

fn rec(m: i64, floors: &Vec<(i64, i64)>, pos: (i64, i64)) -> i64 {
    if pos.1 == -1 {return 0;}
    let travelled = if floors[pos.1 as usize].0 == -1 {
        0
    } else if pos.0 == 0 {
        i64::max(floors[pos.1 as usize].0, floors[pos.1 as usize].1)
    } else {
        i64::max(pos.0 - floors[pos.1 as usize].0, pos.0 - floors[pos.1 as usize].1)
    };

    let p = (
        rec(m, floors, (m-pos.0-1, pos.1-1)),
        rec(m, floors, (pos.0, pos.1-1)),
    );
    let pe = (m-1, travelled + travelled);
    if p.0 == 0 && p.1 == 0 {
        return travelled;
    } else {
        return i64::min(p.0+pe.0 , p.1+pe.1) + 1;
    }
}
