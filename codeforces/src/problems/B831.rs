
use std::io::{BufWriter, stdin, stdout, Write};
 
#[derive(Default)]
struct Scanner {
    buffer: Vec<String>
}
impl Scanner {
    fn next(&mut self) -> String {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token;
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

    let layout_1 = s.next().chars().collect::<Vec<char>>();
    let layout_2 = s.next().chars().collect::<Vec<char>>();
    let string_to_convert = s.next().chars().collect::<Vec<char>>();

    for c in string_to_convert {
        if c.is_alphabetic() {
            
            let k = if c.is_uppercase() {
                let index = layout_1.iter().position(|&k| k == c.to_lowercase().to_string().chars().next().unwrap()).unwrap();
                layout_2[index].to_uppercase().to_string().chars().next().unwrap()
            } else {
                let index = layout_1.iter().position(|&k| k == c).unwrap();
                layout_2[index]
            };
            write!(out, "{k}").unwrap();
        } else {
            write!(out, "{c}").unwrap();
        }
    }

    // dbg!(layout_1[0].is_uppercase(), layout_1[0].is_alphabetic(), &layout_1, layout_2, string_to_convert);
}