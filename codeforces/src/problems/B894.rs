
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

/*
if k == -1 and only 1 of m and n a=is even/odd -> no possible way -> return 0

last row and column can alone change the value of the row/column,
so the number required is 2^(m-1 + n-1) % 1_000_000_007
*/
#[allow(unreachable_code)]
fn main() {
    panic!(); // (a^b)%c is too difficult


    let mut s = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let n = s.next::<u64>();
    let m = s.next::<u64>();
    let k = s.next::<i64>();

    if k == -1 && n%2 != m%2 {
        writeln!(out, "{}", 0).unwrap();
        return;
    }

    // let mut a = 1_u64;
    let c = 1_000_000_007;// << 0;
    // for _ in 0..((n-1)*(m-1)) {
    //     a = a << 1;
    //     a %= c;
    //     // if a/c > 3 {
    //     //     a -= c * (a/c -1);
    //     // }
    // }
    // writeln!(out, "{}", a%1_000_000_007).unwrap();
    writeln!(out, "{}", exponent_mod(2, (n-1)*(m-1), c)).unwrap();
}

fn exponent_mod(a: u64, b: u64, c: u64) -> u64 {
    if a == 0 {return 0;}
    if b == 0 {return 1;}
    let y = if b%2 == 0 {
        let y = exponent_mod(a, b/2, c);
        (y*y)%c
    } else {
        let y = a%c;
        (y*exponent_mod(a, b-1, c)%c)%c
    };

    (y + c)%c
}

/* https://www.geeksforgeeks.org/modular-exponentiation-recursive/
int exponentMod(int A, int B, int C)
{
    // Base cases
    if (A == 0)
        return 0;
    if (B == 0)
        return 1;
 
    // If B is even
    long y;
    if (B % 2 == 0) {
        y = exponentMod(A, B / 2, C);
        y = (y * y) % C;
    }
 
    // If B is odd
    else {
        y = A % C;
        y = (y * exponentMod(A, B - 1, C) % C) % C;
    }
 
    return (int)((y + C) % C);
}
*/