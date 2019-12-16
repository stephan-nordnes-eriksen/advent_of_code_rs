use std::fs::File;
use std::io::{BufRead, BufReader};

fn opcode_plus(data: &mut Vec<i32>, position_a: i32, position_b: i32, store_position: i32) {
    data[store_position as usize] = data[position_a as usize] + data[position_b as usize];
}
fn opcode_multiply(data: &mut Vec<i32>, position_a: i32, position_b: i32, store_position: i32) {
    data[store_position as usize] = data[position_a as usize] * data[position_b as usize];
}
fn run_program(data: &mut Vec<i32>, noun: i32, verb: i32, desired_result: i32) -> bool {
    // Run program
    let mut done = false;
    data[1] = noun;
    data[2] = verb;
    let data_copy = data.to_vec();
    let data_chunks = data_copy.chunks(4);
    for data_point in data_chunks {
        // let data_ref = &mut data;
        if !done {
            match data_point[0] {
                1 => opcode_plus(data, data_point[1], data_point[2], data_point[3]),
                2 => opcode_multiply(data, data_point[1], data_point[2], data_point[3]),
                99 => done = true,
                _ => done = true // some kind of error
            }
        }
    }
    return desired_result == data[0];
}
pub fn advent() {
    let filename = "./data/advent_2_1.data";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut original_data = Vec::new();
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let string_array: Vec<&str> = line.split(",").collect();
        let mut current_position = 0;
        original_data.resize(string_array.len(), 0);
        for string_number in &string_array {
            original_data[current_position] = string_number.parse::<i32>().unwrap();
            current_position += 1;
        }
    }
    
    let mut solution_found = false;
    for noun in 0..99 {
        for verb in 0..99 {
            let mut cloned_data = original_data.clone();
            solution_found = run_program(&mut cloned_data, noun, verb, 19690720);
            if solution_found { 
                println!("Solution found!! Verb {}, Noun {}", verb, noun);
                println!("Final solution: {}", (100 * noun) + verb);
                break; 
            }
        }
        if solution_found { break; }
    }
    
}
