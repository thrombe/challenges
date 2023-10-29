
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
    let l = s.next::<u64>();
    let r = s.next::<u64>();

    writeln!(out, "{}", one_count(n, l, r)).unwrap();
}

fn one_count(n: u64, l: u64, r: u64) -> u64 {
    let mut count = 0;
    let mut current_index = 1;
    let mut num_stack = vec![n];
    while num_stack.len() != 0 {
        let m = num_stack.pop().unwrap();
        if m < 2 {
            if current_index > r {break;}
            if current_index >= l {
                count += m;
            }
            current_index += 1;
            continue;
        }
        let exp_len = get_expansion_length(m);
        if current_index >= l && current_index + exp_len <= r {
            count += m; // no of 1s in the complete expansion of m is m
            current_index += exp_len;
        } else if current_index + exp_len < l {
            current_index += exp_len;
        } else { // if the expansion of m spans across the starting/ending boundary, go another level deep in expansion
            let mid = m%2;
            let mb2 = m >> 1;
            num_stack.push(mb2);
            num_stack.push(mid);
            num_stack.push(mb2);
        }
    }
    count
}

fn get_expansion_length(n: u64) -> u64 {
    (1 << (power_of_2_gte(n) + if n.count_ones() > 1 {0} else {1})) - 1
}


fn power_of_2_gte<T>(n: T) -> usize
where T: Into<u64>
{
    let n = n.into();
    // the smallest power of 2 greater than or equal to n
    let power = 64 
        + if n.count_ones() > 1 || n < 2 {1} else {0} // this in the middle as if n == 0, it still works
        - n.leading_zeros()-1;
    power as usize
}
