use std::cmp;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl fmt::Debug for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Direction::Right => write!(f, "Right"),
            Direction::Left => write!(f, "Left"),
            Direction::Up => write!(f, "Up"),
            Direction::Down => write!(f, "Down"),
        }
    }
}

type AdventOperation = (Direction, i32);

pub fn advent() {
    let filename = "./data/advent_3_2.data";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut map1_operations: Vec<(Direction, i32)> = Vec::new();
    let mut map2_operations: Vec<(Direction, i32)> = Vec::new();
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let string_array: Vec<AdventOperation> = line
            .split(",")
            .map(|s| match &s[..1] {
                "R" => (Direction::Right, s[1..].parse().unwrap()),
                "L" => (Direction::Left, s[1..].parse().unwrap()),
                "U" => (Direction::Up, s[1..].parse().unwrap()),
                "D" => (Direction::Down, s[1..].parse().unwrap()),
                _ => (Direction::Right, 0),
            })
            .collect();
        match index {
            0 => map1_operations = string_array,
            1 => map2_operations = string_array,
            _ => println!("Invalid map found"),
        }
    }

    let mapSizeUsize = 10000;
    let mapSize: i32 = mapSizeUsize as i32;
    let mut map = vec![vec![-1; mapSizeUsize * 2]; mapSizeUsize * 2];
    let mut current_x = mapSizeUsize;
    let mut current_y = mapSizeUsize;
    let mut max_current_x = mapSize;
    let mut max_current_y = mapSize;
    let mut min_current_x = mapSize;
    let mut min_current_y = mapSize;
    let mut current_distance: i32 = 0;
    for tpl_map_one in map1_operations {
        for _i in 0..(tpl_map_one.1) {
            if(map[current_x][current_y] == -1) {
                map[current_x][current_y] = current_distance;
            }
            match tpl_map_one.0 {
                Direction::Right => current_x += 1,
                Direction::Left => current_x -= 1,
                Direction::Up => current_y += 1,
                Direction::Down => current_y -= 1,
            }
            max_current_x = cmp::max(current_x as i32, max_current_x);
            max_current_y = cmp::max(current_y as i32, max_current_y);
            min_current_x = cmp::min(current_x as i32, min_current_x);
            min_current_y = cmp::min(current_y as i32, min_current_y);
            current_distance += 1;
        }
    }
    current_x = mapSizeUsize;
    current_y = mapSizeUsize;

    let mut distance = 10 * mapSize * mapSize;
    let mut temp_distance = 0;
    let mut current_distance_map_two = 0;
    for tpl_map_two in map2_operations {
        for _i in 0..(tpl_map_two.1) {
            if !(current_x as i32 == mapSize && current_y as i32 == mapSize) {
                if map[current_x][current_y] != -1 {
                    temp_distance = current_distance_map_two + map[current_x][current_y];
                    if(temp_distance < distance){
                        distance = temp_distance.clone();
                        println!(
                            "X here x:{}, y:{}, Distance: {}",
                            current_x, current_y, distance
                        );
                    }
                }
            }
            match tpl_map_two.0 {
                Direction::Right => {
                    current_x += 1;
                }
                Direction::Left => {
                    current_x -= 1;
                }
                Direction::Up => {
                    current_y += 1;
                }
                Direction::Down => {
                    current_y -= 1;
                }
            }
            current_distance_map_two += 1;
        }
    }
    println!("Distance {:?}", distance);
    println!(
        "Max: {}, {}. Min: {}, {}",
        max_current_x, max_current_y, min_current_x, min_current_y
    );
}
