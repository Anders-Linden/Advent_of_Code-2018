use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

// Helpers
fn open_input() -> std::io::BufReader<std::fs::File> {
    let file = match File::open("./assets/input") {
            Err(why) => panic!("couldn't open, {}", why),
            Ok(file) => file,
        };
    return BufReader::new(file);
}

//
// Functions
//

// Find resulting frequency
fn part1() -> i32 {
    let init_freq = 0;
    let mut result = init_freq;
    
    for line in open_input().lines() {
        let freq_change = line.unwrap().parse().unwrap();
        result = &result+&freq_change;
    }
    return result;
}

// Find first recuring freqency
fn part2() -> i32 {
    let init_freq = 0;
    let mut result = init_freq;
    let mut resulting_freq: Vec<i32> = Vec::new();

    loop {
        for line in open_input().lines() {
            let freq_change = line.unwrap().parse().unwrap();   
            result = &result+&freq_change;
            
            if resulting_freq.iter().any(|&x| x == result) {
                return result;
            }
            resulting_freq.push(result);
        }
    }
}

fn main() {
    println!("Results for Day 1");
    println!("-------------------");
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}