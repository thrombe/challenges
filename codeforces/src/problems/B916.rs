
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
    let k = s.next::<i32>();

    let mut bit_powers = vec![0;64];
    for i in 0..(64-n.leading_zeros()) as usize {
        if n & (1 << i) > 0 {
            bit_powers[i] = 1;
        }
    }

    // -ve part of this vec is not being used. im lazy to refactor it
    let mut powers = (-63..63).map(|i| if i < 0 {0} else if n & (1 << i) > 0 {1} else {0}).collect::<Vec<i32>>();
    let mut m = powers.iter().filter(|i| **i == 1).count() as i32;
    if m > k {
        writeln!(out, "No").unwrap();
        return;
    }

    let mut maybe_iter = None;
    while m != k {
        let (i, _) = powers.iter().enumerate().filter(|(_, k)| **k > 0).rev().next().unwrap();
        if m + powers[i] <= k {
            powers[i-1] += 2*powers[i];
            m += powers[i];
            powers[i] = 0;
        } else {
            // dbg!(powers.iter().enumerate().map(|(i, k)| (i as i32-63, k)).collect::<Vec<(i32, &i32)>>());
            let (i, _) = powers.iter().enumerate().filter(|(i, k)| **k > 0 && *i > 0).next().unwrap();
            m -= 1;
            maybe_iter = Some((i..k as usize+i).into_iter().map(move |p| {
                if m == k-1 {
                    m += 1;
                    Some(2*(i as i32-63) + (63-p as i32))
                } else if m != k {
                    m+=1;
                    Some(2*(i as i32 - 63) + (63-p as i32) - 1)
                } else {
                    None
                }
            }).filter(|o| o.is_some()).map(|o| o.unwrap()));
            powers[i] -= 1;
            break;
        }
    }
    writeln!(out, "Yes").unwrap();
    // dbg!(powers.iter().enumerate().map(|(i, k)| (i as i32-63, k)).collect::<Vec<(i32, &i32)>>());
    let it = powers.into_iter()
        .enumerate()
        .rev()
        .filter(|(_, k)| *k > 0)
        .map(|(i, k)| (0..k).into_iter().map(move |k| (i, k)))
        .flatten()
        .map(|(i, _)| i as i32-63);
    if maybe_iter.is_some() {
        it.chain(maybe_iter.unwrap()).for_each(|i| write!(out, "{i} ").unwrap());
    } else {
        it.for_each(|i| write!(out, "{i} ").unwrap());
    }
}


fn _main2() {
    let mut s = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let n = s.next::<u64>();
    let k = s.next::<usize>();

    let mut bit_powers = vec![];
    for i in 0..(64-n.leading_zeros()) {
        if n & (1 << i) > 0 {
            bit_powers.push(i as i32);
        }
    }

    let mut bit_powers_stack = vec![bit_powers.pop().unwrap()];
    while bit_powers.len()+bit_powers_stack.len() < k {
        // pick the biggest power with least index
        if bit_powers_stack.len() == 0 {
            bit_powers_stack.push(bit_powers.pop().unwrap());
        }
        while bit_powers.len() > 0 && bit_powers[bit_powers.len()-1] == bit_powers_stack[0] {
            bit_powers_stack.push(bit_powers.pop().unwrap());
        }
        let p = bit_powers_stack.pop().unwrap();
        bit_powers.push(p-1);
        bit_powers.push(p-1);
    }
    if bit_powers.len()+bit_powers_stack.len() > k {
        // dbg!(&bit_powers, &bit_powers_stack);
        writeln!(out, "No").unwrap();
        return;
    }
    
    writeln!(out, "Yes").unwrap();
    // dbg!(&bit_powers.len(), &bit_powers_stack.len());
    // let mut num = 0;
    // bit_powers_stack.iter().chain(bit_powers.iter().rev()).enumerate()
    //     .for_each(|(i, k)| {
    //         // write!(out, "{i} ").unwrap();
    //         num += 1_u64 << *k;
    //         match i {
    //             99970..=1000000 => {dbg!(i, k);},
    //             _ => (),
    //         }
    //     });
    // dbg!(num);
    bit_powers_stack.into_iter().chain(bit_powers.into_iter().rev())
    .for_each(|i| {
        write!(out, "{i} ").unwrap(); // does not give the lexicographically largest one
    });
}