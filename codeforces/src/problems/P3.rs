
use std::io::{BufWriter, stdin, stdout, Write};
 
#[derive(Default)]
struct Scanner {
    buffer: Vec<String>
}
impl Scanner {
    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                // dbg!(&token);
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

    if n < 2 || m < 2 {
        writeln!(out, "0").unwrap();
        return;
    }

    let board: Vec<Vec<char>> = (0..n).into_iter().map(
        |_| s.next::<String>().chars().collect()
    ).collect();

    let mut count = 0;
    for i in 0..(n-1) {
        for j in 0..(m-1) {
            let mut face = vec!['f', 'a', 'c', 'e'];
            face.retain(|&v| v != board[i][j]);
            face.retain(|&v| v != board[i+1][j]);
            face.retain(|&v| v != board[i][j+1]);
            face.retain(|&v| v != board[i+1][j+1]);
            if face.len() == 0 {
                count += 1;
            }
        }
    }

    writeln!(out, "{count}").unwrap();
}