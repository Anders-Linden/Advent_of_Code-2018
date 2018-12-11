use std::collections::BTreeMap;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

// Helpers
fn open_input() -> std::io::BufReader<std::fs::File> {
    let file = match File::open("./assets/input") {
        Err(why) => panic!("couldn't open, {}", why),
        Ok(file) => file,
    };
    BufReader::new(file)
}

//
// Functions
//

// Find rudimentary checksum
fn part1() -> u32 {
    let mut result = 1;
    let mut rudimentary_factors: HashMap<u32, u32> = HashMap::new();
    for line in open_input().lines() {
        let box_id = line.unwrap();
        let mut occurrence: HashMap<char, u32> = HashMap::new();
        for letter in box_id.chars() {
            *occurrence.entry(letter).or_insert(0) += 1;
        }

        let mut rudimentary_part: HashMap<u32, u32> = HashMap::new();
        for (_key, value) in occurrence.into_iter() {
            if value > 1 {
                *rudimentary_part.entry(value).or_insert(0) = 1;
            }
        }
        for (key, value) in rudimentary_part.into_iter() {
            *rudimentary_factors.entry(key).or_insert(0) += value;
        }
    }
    for (_key, value) in rudimentary_factors.into_iter() {
        result *= value;
    }
    result
}

// Find common letters in Box IDs letters
fn part2() -> (usize, String) {
    let mut box_id_match: BTreeMap<usize, String> = BTreeMap::new();
    let box_id: Vec<_> = open_input().lines().map(|res| res.unwrap()).collect();
    let num_elements = box_id.len();

    for (i, element) in box_id.iter().enumerate() {
        for compare_id in &box_id[i + 1..num_elements] {
            let mut letter_match = String::new();
            let mut compare_char = compare_id.chars();
            for letter in element.chars() {
                if compare_char.next().unwrap() == letter {
                    letter_match.push(letter);
                }
            }
            box_id_match.insert(letter_match.len(), letter_match);
        }
    }
    let (num_common, letters_in_common)  = box_id_match.iter().next_back().unwrap();

    (*num_common, letters_in_common.to_string())
}

fn main() {
    println!("Results for Day 2");
    println!("-------------------");
    println!("Part 1: {}", part1());
    let (num, letters) = part2();
    println!("Part 2: Number of letters {}", num);
    println!("Answer: {}", letters)
}
