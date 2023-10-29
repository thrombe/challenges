
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

    let mut board = (0..9).map(
        |_| (0..3).map(
            |_| s.next::<String>().chars().collect::<Vec<char>>()
        ).flatten().collect::<Vec<char>>()
    ).collect::<Vec<Vec<char>>>();
    let coords = (s.next::<usize>()-1, s.next::<usize>()-1);

    let mut found_any = false;
    for i in 0..3 {
        for j in 0..3 {
            let x = ((coords.1%3)*3 + j)%9;
            let y = ((coords.0%3)*3 + i)%9;
            if board[y][x] == '.' {
                found_any = true;
                board[y][x] = '!';
            }
        }
    }

    board.into_iter().enumerate().for_each(|(j, l)| {
        l.into_iter().enumerate().for_each(|(i, mut e)| {
            if !found_any {
                if e == '.' {
                    e = '!';                    
                }
            }
            if (i+1)%3 == 0 {
                write!(out, "{e} ").unwrap();
            } else {
                write!(out, "{e}").unwrap();
            }
        });
        write!(out, "{}", if (j+1)%3 == 0 {"\n\n"} else {"\n"}).unwrap();
    });
}