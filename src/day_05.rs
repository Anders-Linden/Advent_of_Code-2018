use std::io::prelude::*;
extern crate utilities as utils;

fn compare_chars(formula: &Vec<u8>) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();

    for character in formula {
        if result.is_empty() {
            result.push(*character);
        } else if ((*result.last().unwrap() as i8) - (*character as i8) as i8).abs() as u8 == 32u8 {
            result.pop();
        } else {
            result.push(*character);
        }
    }
    result
}
fn compare_single_chars(formula: &Vec<u8>, limit: u8) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    // Array of [upper case, lower case]
    let limit_list = [limit, limit + 32u8];

    for character in formula {
        if !limit_list.contains(character) {
            result.push(*character);
        }
    }
    result
}

fn part1(data : &Vec<u8>) -> usize {
    let compare_data = compare_chars(&data);

    compare_data.len() - 1
}

fn part2(data : &Vec<u8>) -> usize {
    let mut compare = Vec::new();

    // ASCII Code for "A" is 65, for "a" it is A + 32 = 97
    // "B" = 66, "b" = B+32 = 98
    for plus in 0..26 {
        // Going through A to Z
        let data_a = compare_chars(&compare_single_chars(&data, 65u8 + plus));
        compare.push(data_a.len() - 1);
    }

    let value = compare.iter().min_by(|a, b| a.cmp(b));

    *value.unwrap()
}

fn main() {
    let mut buf_reader = utils::open_input("./assets/input_day05");
    let mut data = Vec::new();
    match buf_reader.read_to_end(&mut data) {
        Ok(_data) => (),
        Err(er) => println!("I/O Error: {}", er),
    }

    println!("Results for Day 5");
    println!("-------------------");
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn code_coverage_test_main() {
        assert_eq!(main(), ());
    }
}