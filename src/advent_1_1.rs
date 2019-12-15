use std::fs::File;
use std::io::{BufRead, BufReader};
pub fn advent() {
    let filename = "./advent_1_1.data";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut sum = 0;
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let number = line.parse::<i32>().unwrap();
        sum += (number / 3) - 2;
    }
    println!("Sum fuel {}.", sum);
}
