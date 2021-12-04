
use crate::utils::*;

#[test]
pub fn find_password() {
    dbg!(is_password(111111));
    dbg!(is_password(223450));
    dbg!(is_password(123789));
    dbg!(is_password(123444));
    dbg!(is_password(111122));
    
    
    let mut count = 0;
    for x in 372304..=847060 {
        if is_password(x) {
            count += 1;
        }
    }
    dbg!(count);
}

pub fn is_password(x: usize) -> bool {
    let str = x.to_string();
    if str.len() != 6 {
        panic!("not a 6 digit int {}", x);
    }
    let digits: Vec<usize> = str.split("").filter(|s| s.len() > 0).map(|s| s.parse().unwrap()).collect();
    let mut last_dig = 0;
    let mut had_double = false;
    let mut run = 0;
    for digit in digits {
        if digit < last_dig {
            return false
        }


        if digit == last_dig {
            run += 1;
        } else {
            if run == 2 {
                had_double = true
            }
            run = 1;
        }

        last_dig = digit
    }

    if run == 2{
        had_double = true
    }

    had_double
}