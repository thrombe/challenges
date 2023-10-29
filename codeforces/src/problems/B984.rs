
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

    let n = s.next::<usize>();
    let m = s.next::<usize>();
    let mut board = vec![vec![0;m];n];

    for i in 0..n {
        for (j, c) in s.next::<String>().chars().enumerate() {
            match c {
                '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' => {
                    board[i][j] += c as i64 - '0' as i64;
                },
                '.' => {}, // already set to 0
                '*' => {
                    board[i][j] += 9000; // ~~9000 represents bomb
                    for (x, y) in [(-1, 1), (0, 1), (1, 1), (1, 0), (1, -1), (0, -1), (-1, -1), (-1, 0)] {
                        let index = (j as i64 + y, i as i64 + x);
                        if index.0 >= m as i64 || index.0 < 0 || index.1 >= n as i64 || index.1 < 0 {continue;}
                        board[index.1 as usize][index.0 as usize] -= 1;
                    }
                },
                _ => panic!(),
            }
        }
    }
 
    for i in 0..n {
        for j in 0..m {
            if board[i][j] != 0 && board[i][j] < 1000 {
                writeln!(out, "NO").unwrap();
                return;
            }
        }
    }

    writeln!(out, "YES").unwrap();
}