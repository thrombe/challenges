
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

    s.next::<usize>();
    let k = s.next::<usize>();
    let guest_list = s.next::<String>().chars().collect::<Vec<char>>();

    let mut intervals = [(-1, -1);26];
    for (p, c) in guest_list.iter().enumerate() {
        let index = *c as usize - 'A' as usize;
        if intervals[index].0 == -1 {
            intervals[index].0 = p as i64;
            intervals[index].1 = p as i64;
        } else {
            intervals[index].1 = p as i64;
        }
    }
    // dbg!(intervals);

    let mut open_count = 0;
    for (p, c) in guest_list.iter().enumerate() {
        let index = *c as usize - 'A' as usize;
        if intervals[index].0 == p as i64 {
            open_count += 1;
            if open_count > k {
                writeln!(out, "YES").unwrap();
                return;
            }
        }
        if intervals[index].1 == p as i64 {
            open_count -= 1;
        }
    }
    writeln!(out, "NO").unwrap();
}