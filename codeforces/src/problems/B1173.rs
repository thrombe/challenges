
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

    let n = s.next::<u64>();
    
    // |xi - xj| + |yi - yj| <= 2*(m-1) // as coords can be (1, 1) and (m, m)
    //                       <= n-1 // as values of items can be n, 1
    // min m = (n+1)/2
    // n = 2m-1 -> place items on the diagonal

    writeln!(out, "{}", (n)/2 + 1).unwrap(); // n/2 + 1 instead of (n+1)/2 as it has to be an integer and (n+1)/2 is the min value
    let mut x = 1;
    let mut y = 1;
    for i in 0..n {
        writeln!(out, "{x} {y}").unwrap();


        if i%2 == 0 {
            x += 1;
        } else {
            y += 1;
        }
    }
}