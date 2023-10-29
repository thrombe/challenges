
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

    let good_chars = s.next::<String>().chars().collect::<Vec<char>>();
    let pattern = s.next::<String>();//.chars().collect::<Vec<char>>();
    let patterns = pattern.split("*").collect::<Vec<&str>>();

    'current_query: for _ in 0..s.next::<u64>() {
        let string = s.next::<String>().chars().collect::<Vec<char>>();
        if patterns.len() == 1 {
            for (i, c) in pattern.chars().enumerate() {
                if string.len() != pattern.len() {
                    writeln!(out, "NO").unwrap();
                    continue 'current_query;
                }
    
                if c != string[i] {
                    match c {
                        '?' => {
                            if !good_chars.contains(&string[i]) {
                                writeln!(out, "NO").unwrap();
                                continue 'current_query;
                            }
                        },
                        _ => {
                            writeln!(out, "NO").unwrap();
                            continue 'current_query;
                        },
                    }
                }
            }
        } else {
            if string.len() < patterns[0].len() + patterns[1].len() {
                writeln!(out, "NO").unwrap();
                continue 'current_query;
            }

            for (i, c) in patterns[0].chars().enumerate() {
                if c != string[i] {
                    match c {
                        '?' => {
                            if !good_chars.contains(&string[i]) {
                                writeln!(out, "NO").unwrap();
                                continue 'current_query;
                            }
                        },
                        _ => {
                            writeln!(out, "NO").unwrap();
                            continue 'current_query;
                        },
                    }
                }
            }

            for i in patterns[0].len()..(string.len()-patterns[1].len()) {
                if good_chars.contains(&string[i]) {
                    writeln!(out, "NO").unwrap();
                    continue 'current_query;
                }
            }

            for (i, c) in patterns[1].chars().rev().enumerate() {
                let i = string.len() as i64-i as i64 -1;
                if c != string[i as usize] {
                    match c {
                        '?' => {
                            if !good_chars.contains(&string[i as usize]) {
                                writeln!(out, "NO").unwrap();
                                continue 'current_query;
                            }
                        },
                        _ => {
                            writeln!(out, "NO").unwrap();
                            continue 'current_query;
                        },
                    }
                }
            }
        }
        writeln!(out, "YES").unwrap();
    }
}



fn sssss() {
    let mut s = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let good_chars = s.next::<String>().chars().collect::<Vec<char>>();
    let pattern = s.next::<String>().chars().collect::<Vec<char>>();

    'current_query: for _ in 0..s.next::<u64>() {
        let string = s.next::<String>().chars().collect::<Vec<char>>();
        // dbg!(&string);
        let len = string.len();
        if (len as i64 - pattern.len() as i64) < -1 {
            writeln!(out, "NO").unwrap();
            continue 'current_query;    
        }
        let mut i = -1_i64;
        let mut string = string.into_iter().enumerate();
        'loup: loop {
            let (j, k) = match string.next() {
                Some(p) => p,
                None => break,
            };
            i += 1;
            // dbg!(k, i, j, pattern[i as usize]);
            if k != pattern[i as usize] {
                match pattern[i as usize] {
                    '?' => {
                        if !good_chars.contains(&k) {
                            writeln!(out, "NO").unwrap();
                            continue 'current_query;
                        }
                    },
                    '*' => {
                        let di = len as i64-pattern.len() as i64;
                        // dbg!(di);
                        if di == -1 {
                            if j == len-1 {
                                i += 1;
                                continue 'loup;
                            } else if k == pattern[i as usize + 1] {
                                i += 1;
                                continue 'loup;
                            } else {
                                writeln!(out, "NO").unwrap();
                                continue 'current_query;    
                            }
                        }
                        for _ in 0..di {
                            let (_j, k) = match string.next() {
                                Some(p) => p,
                                None => {
                                    // if i != pattern.len() as i64-1 {
                                    //     writeln!(out, "NO").unwrap();
                                    //     continue 'current_query;
                                    // }
                                    // break 'loup;
                                    panic!();
                                },
                            };
                            if good_chars.contains(&k) {
                                writeln!(out, "NO").unwrap();
                                continue 'current_query;        
                            }
                        }
                    },
                    _ => {
                        // dbg!(k, _j, i, pattern[i as usize]);
                        writeln!(out, "NO").unwrap();
                        continue 'current_query;
                    }
                }
            }
        }
        if i == pattern.len() as i64 -1 || (pattern[i as usize+1] == '*') {
            writeln!(out, "YES").unwrap();
        } else {
            // dbg!(i, pattern.len());
            writeln!(out, "NO").unwrap();
        }
    }
}