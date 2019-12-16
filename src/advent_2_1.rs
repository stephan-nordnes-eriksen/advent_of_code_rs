use std::fs::File;
use std::io::{BufRead, BufReader};

fn opcode_plus(data: &mut Vec<i32>, position_a: i32, position_b: i32, store_position: i32) {
    data[store_position as usize] = data[position_a as usize] + data[position_b as usize];
}
fn opcode_multiply(data: &mut Vec<i32>, position_a: i32, position_b: i32, store_position: i32) {
    data[store_position as usize] = data[position_a as usize] * data[position_b as usize];
}
pub fn advent() {
    let filename = "./data/advent_2_1.data";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    
    let mut data = Vec::new();

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let string_array: Vec<&str> = line.split(",").collect();
        let mut current_position = 0;
        data.resize(string_array.len(), 0);
        for string_number in &string_array {
            data[current_position] = string_number.parse::<i32>().unwrap();
            current_position += 1;
        }
    }
    // Run program
    let mut done = false;
    let data_copy = data.to_vec();
    let data_chunks = data_copy.chunks(4);
    for data_point in data_chunks {
        let data_ref = &mut data;
        if !done {
            match data_point[0] {
                1 => opcode_plus(data_ref, data_point[1], data_point[2], data_point[3]),
                2 => opcode_multiply(data_ref, data_point[1], data_point[2], data_point[3]),
                99 => done = true,
                _ => done = true // some kind of error
            }
        }
    }
    // println!("Result program {}", data.join(","));
    println!("Program done {:?}", done);
    println!("Result program {:?}", data);

}
