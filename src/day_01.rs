use std::io::prelude::*;
extern crate utilities as utils;
//
// Functions
//

// Find resulting frequency
fn part1() -> i32 {
    let init_freq = 0;
    let mut result = init_freq;

    for line in utils::open_input("./assets/input_day01").lines() {
        let freq_change: i32 = line.unwrap().parse().unwrap();
        result += freq_change;
    }
    result
}

// Find first recuring freqency
fn part2() -> i32 {
    let init_freq = 0;
    let mut result = init_freq;
    let mut resulting_freq: Vec<i32> = Vec::new();

    loop {
        for line in utilities::open_input("./assets/input_day01").lines() {
            let freq_change: i32 = line.unwrap().parse().unwrap();
            result += freq_change;

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
