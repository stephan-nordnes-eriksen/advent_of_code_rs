
fn test_password(password: i32) -> bool {
    let pwd_string = password.to_string();
    let pwd_parts: Vec<i32> = pwd_string.split("").filter(|x| x != &"").map(|x| x.parse().unwrap()).collect();
    let mut last_pos = 0;
    let mut always_increasing = true;
    let mut two_equal = false;
    for number_pos in pwd_parts {
        if last_pos > number_pos {
            always_increasing = false;
        }
        if last_pos == number_pos {
            two_equal = true;
        }
        last_pos = number_pos;
    }
    return always_increasing && two_equal;
}

pub fn advent() {
    println!("Test password: TRUE: 111111 : {}", test_password(111111));
    println!("Test password: FALSE: 223450 : {}", test_password(223450));
    println!("Test password: FALSE: 123789 : {}", test_password(123789));
    let mut count = 0;
    for password in 402328..864247 {
        if test_password(password) {
            count += 1;
        }
    }
    println!("Count: {}", count);
}
