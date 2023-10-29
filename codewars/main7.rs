
fn main() {
    println!("{:?}", last_digit(&[2, 2, 101, 2]));
    // println!("{:?}", last_digit2("2", "49"));
}
!! dosent work

// https://www.codewars.com/kata/5518a860a73e708c0a000027/train/rust

fn last_digit(lst: &[u64]) -> u64 {
    if lst.len() == 0 {return 1}
    if lst.len() == 1 {
        return lst[0] % 10;
    }
    // dbg!(&lst);
    let mut lst = lst.into_iter().rev();
    let mut t = *lst.next().unwrap();

    for _ in 0..lst.len() {
        let mut modded = t%1000;
        if t != 0 && modded == 0 {
            modded = 1000;
        }    
        t = last_digit2(*lst.next().unwrap(), modded);
    }
    t % 10
}

fn last_digit2(num1: u64, num2: u64) -> u64 {
    dbg!(num1, num2, "");
    if num2 == 0 {return 1}
    if num1 == 1 {return 1}
    if num2 == 1 {return num1}
    let mut last_digits = vec![];
    let mut d = num1;
    for _i in 2.. {
        d *= num1;
        d = d%1000;
        let modded = d%1000;
        if !last_digits.contains(&modded) {
            last_digits.push(modded);
        } else {
            break;
        }
    }
    if last_digits.len() == 1 {
        if last_digits[0] == 0 {return 1000}
        return last_digits[0];
    }
    
    let pow: usize = num2 as usize;
    let index = (pow) % last_digits.len();

    // dbg!(&index, &num2, &last_digits);
    // instead of mod, i needed something that maps from 1 to n eg : 1%%4=1, 4%%4=4, 5%%4=1 (%% being the custom func)
    // if index == 0 {return last_digits[last_digits.len()-1]}
    let index = if index as i32-2 < 0 {index+last_digits.len()-2} else {index-2};
    last_digits[index]
}
























// fn last_digit(str1: &str, str2: &str) -> i32 {
//     if str2 == "0" {return 1}
//     let mut last_digits = vec![];
//     let last_d = str1[str1.len()-1..].parse::<i32>().unwrap();
//     last_digits.push(last_d);
//     for i in 2..5 {
//         let d = last_d.pow(i );
//         if !last_digits.contains(&(d % 10)) {
//             last_digits.push(d % 10);
//         } else {
//             break;
//         }
//     }
//     if last_digits.len() == 1 {return last_digits[0]}
    
//     let pow: usize = (if str2.len() > 2 {&str2[str2.len()-2..]} else {&str2[..]}).parse().unwrap();
//     let index = (pow) % last_digits.len();

//     dbg!(&index, &last_d, &last_digits);
//     // instead of mod, i needed something that maps from 1 to n eg : 1%%4=1, 4%%4=4, 5%%4=1 (%% being the custom func)
//     if index == 0 {return last_digits[last_digits.len()-1]}
//     last_digits[index-1]
// }