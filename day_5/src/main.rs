use std::fs::File;
use std::io::prelude::*;

// Helpers
fn open_input() -> std::fs::File {
    let file = match File::open("./assets/input") {
            Err(why) => panic!("couldn't open, {}", why),
            Ok(file) => file,
        };
    file
}

fn compare_chars (formula: Vec<u8>) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();

    for character in formula {
        if result.is_empty() {
            result.push(character);
        } else if ((*result.last().unwrap() as i8)  - (character as i8) as i8).abs() as u8  == 32u8 {
            result.pop();
        } else{
            result.push(character);
        }
    }
    result
}


fn part1() -> usize {
    let mut reader = open_input();
    let mut buffer = Vec::new();

    match reader.read_to_end(&mut buffer) {
        Ok(_data) => (),
        Err(er) => println!("I/O Error: {}", er),
    }

    let data =  compare_chars(buffer);
    
    data.len()-1
}

fn main() {
    println!("Results for Day 5");
    println!("-------------------");
    println!("Part 1: {}",part1());
}
