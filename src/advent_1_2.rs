use std::fs::File;
use std::io::{BufRead, BufReader};

fn addFuelForMass(mut current_sum: i64, mass: i64) -> i64 {
    let mut new_fuel = (mass/3) -2;
        while (new_fuel > 0) {
            current_sum += new_fuel;
            new_fuel = (new_fuel/3) - 2;
        }
    return current_sum;
}
pub fn advent() {
    let filename = "./advent_1_2.data";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    
    let testSum = addFuelForMass(0, 100756);
    println!("testSum {}", testSum);
    
    let mut sum = 0;
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let number = line.parse::<i64>().unwrap();

        sum = addFuelForMass(sum, number);
        // let mut new_fuel = (number/3) -2;
        // while (new_fuel > 0) {
        //     sum += new_fuel;
        //     new_fuel = (new_fuel/3) - 2;
        // }
    }
    println!("Sum fuel {}", sum);
}
