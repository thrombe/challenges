
fn main() {
    println!("{}", descending_order(12345678));
}

fn descending_order(x: u64) -> u64 {
    let s = format!("{}", x);
    let mut s = s.chars().map(|x| x.to_digit(10).unwrap()).collect::<Vec<u32>>();
    s.sort();
    let mut n = 0;
    for i in 0..s.len() {
        n += (s[i] as u64) * 10_u64.pow(i as u32);
    }
    
    n
}