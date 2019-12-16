
fn test_password(password: i32) -> bool {
    let pwd_string = password.to_string();
    let pwd_parts: Vec<i32> = pwd_string.split("").filter(|x| x != &"").map(|x| x.parse().unwrap()).collect();
    let mut last_pos = 0;
    let mut equal_length = 1;
    let mut always_increasing = true;
    let mut two_equal = false;
    for number_pos in pwd_parts {
        if last_pos > number_pos {
            always_increasing = false;
        }
        if last_pos == number_pos {
            equal_length += 1;
        } else {
            if equal_length == 2 { 
                two_equal = true;
            }
            equal_length = 1;
        }
        last_pos = number_pos;
    }
    if equal_length == 2 { 
        two_equal = true;
    }
    return always_increasing && two_equal;
}

pub fn advent() {
    println!("Test password: FALSE: 111111 : {}", test_password(111111));
    println!("Test password: FALSE: 223450 : {}", test_password(223450));
    println!("Test password: FALSE: 123789 : {}", test_password(123789));
    
    println!("Test password: TRUE: 112233 : {}", test_password(112233));
    println!("Test password: FALSE: 123444 : {}", test_password(123444));
    println!("Test password: TRUE: 111122 : {}", test_password(111122));
    let mut count = 0;
    for password in 402328..864247 {
        if test_password(password) {
            count += 1;
        }
    }
    println!("Count: {}", count);
}
