
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

    let yi = (0..s.next::<i64>()).map(|i| (i+1, s.next::<i64>())).collect::<Vec<(i64, i64)>>();
    if yi.len() == 3 {
        if lies_in_a_line(yi[0], yi[1], yi[2]) {
            write!(out, "No").unwrap();
            return;    
        }
        write!(out, "Yes").unwrap();
        return;
    }
    let mut line_1s = vec![vec![yi[0], yi[1]], vec![yi[0], yi[2]], vec![yi[1], yi[2]]];
    let mut line_2s = vec![vec![yi[2]], vec![yi[1]], vec![yi[0]]];
    
    'main_loup: for _ in 0..3 {
        let mut line_1 = line_1s.pop().unwrap();
        let mut line_2 = line_2s.pop().unwrap();
        if lies_in_a_line(line_1[0], line_1[1], line_2[0]) {
            line_1.push(line_2.pop().unwrap());
        }
        for i in 3..yi.len() {
            if !lies_in_a_line(line_1[0], line_1[1], yi[i]) {
                line_2.push(yi[i]);
            } else {
                line_1.push(yi[i]);
            }
        }
        if line_2.len() == 0 {
            continue 'main_loup;
        }
        if line_2.len() == 1 {
            write!(out, "Yes").unwrap();
            return;
        }
        if (line_1[1].1-line_1[0].1) as f64/(line_1[1].0-line_1[0].0) as f64 != (line_2[1].1-line_2[0].1) as f64/(line_2[1].0-line_2[0].0) as f64 {
            continue;
        }
        

        for i in 2..line_2.len() {
            if !lies_in_a_line(line_2[0], line_2[1], line_2[i]) {
                continue 'main_loup;
            }
        }
        // dbg!(&line_2, &line_1);
        write!(out, "Yes").unwrap();
        return;
    }

    write!(out, "No").unwrap();
}

fn lies_in_a_line(a: (i64, i64), b: (i64, i64), c: (i64, i64)) -> bool {
    let s = (b.1-a.1) as f64/(b.0-a.0) as f64;
    let y = s*(c.0-a.0) as f64;
    a.1 as f64 + y == c.1 as f64
}