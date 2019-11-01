use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::io::prelude::*;
extern crate regex;
use regex::Regex;
extern crate utilities as utils;

#[derive(Debug, Clone, Copy, Default)]
struct Claim {
    id: u32,
    padding_x: u32,
    padding_y: u32,
    width: u32,
    height: u32,
}

impl PartialEq for Claim {
    fn eq(&self, other: &Claim) -> bool {
        self.id == other.id
    }
}

impl Eq for Claim {}

impl Hash for Claim {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

struct Point {
    x: u32,
    y: u32,
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Point {}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

fn calculate_coordinates(claim: Claim) -> Vec<Point> {
    let mut coordinates: Vec<Point> = Vec::new();
    for x_step in 0..claim.width {
        for y_tep in 0..claim.height {
            coordinates.push(Point {
                x: claim.padding_x + x_step,
                y: claim.padding_y + y_tep,
            });
        }
    }
    coordinates
}

fn part1and2() -> (u32, Claim) {
    let mut claims: HashSet<Claim> = HashSet::new();
    let mut claimed_coordinates: HashMap<Point, Vec<Claim>> = HashMap::new();
    let re = Regex::new(
        r"(?si)\#(?P<id>\b[0-9]+) @ (?P<padding_x>\b[0-9]+),(?P<padding_y>\b[0-9]+): (?P<width>\b[0-9]+)x(?P<height>\d+)").unwrap();
    for line in utils::open_input("./assets/input_day03").lines() {
        let str_line = line.unwrap().clone();
        let claim_string = re.captures(&str_line).unwrap();

        let claim = Claim {
            id: claim_string["id"].parse::<u32>().unwrap(),
            padding_y: claim_string["padding_y"].parse::<u32>().unwrap(),
            padding_x: claim_string["padding_x"].parse::<u32>().unwrap(),
            width: claim_string["width"].parse::<u32>().unwrap(),
            height: claim_string["height"].parse::<u32>().unwrap(),
        };
        claims.insert(claim);
        for point in calculate_coordinates(claim) {
            claimed_coordinates
                .entry(point)
                .or_insert_with(Vec::new)
                .push(claim);
        }
    }

    let mut part_one_count = 0;
    let mut part_two_claim = Claim::default();
    for (_point, claim_list) in claimed_coordinates {
        if claim_list.len() > 1 {
            part_one_count += 1;
            for claim in claim_list {
                claims.remove(&claim);
            }
        }
    }

    // Asumed to be only one claim
    for claim in claims {
        part_two_claim = claim;
    }
    (part_one_count, part_two_claim)
}

fn main() {
    println!("Results for Day 3");
    println!("-------------------");
    let (part_one_count, part_two_claim) = part1and2();
    println!("Part 1: {}", part_one_count);
    println!("Part 2: {}", part_two_claim.id);
}
