
fn main() {
    println!("{:?}", dbl_linear(10));
}

fn dbl_linear(n: u32) -> u32{
    let mut s = vec![1];
    let mut i1 = 0;
    let mut y1 = 3;
    let mut i2 = 0;
    let mut y2 = 4;
    loop {
        if s.len() == n as usize + 1 {break}
        if y1 < y2 {
            s.push(y1);
            i1 += 1;
            y1 = f1(s[i1 as usize]);
        } else if y2 < y1 {
            s.push(y2);
            i2 += 1;
            y2 = f2(s[i2 as usize]);
        } else {
            s.push(y1);
            i1 += 1;
            y1 = f1(s[i1 as usize]);
            i2 += 1;
            y2 = f2(s[i2 as usize]);            
        }
    }
    // dbg!(&s);
    s.pop().unwrap()
}

fn f1(x: u32) -> u32 {2*x+1}
fn f2(x: u32) -> u32 {3*x+1}