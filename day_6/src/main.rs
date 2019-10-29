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

#[derive(Debug, PartialEq, Eq, Clone)]
struct Point {
    x: i16,
    y: i16,
}

impl Point {
    fn manhattan_distance(&self, other: &Point) -> u16 {
        //The distance between two points in a grid like street network
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as u16
    }
}

fn get_coordinates() -> Vec<Point> {
    let mut coordinates: Vec<Point> = Vec::new();
    for line in open_input().lines() {
        let spl: Vec<i16> = line
            .unwrap()
            .split(", ")
            .map(|value| value.parse().unwrap())
            .collect();
        coordinates.push(Point {
            x: spl[0],
            y: spl[1],
        })
    }
    return coordinates;
}

fn part1() -> isize {
    let coordinates: Vec<Point> = get_coordinates();
    let mut area_count = vec![0; coordinates.len()];;


    // Boundaries of the world
    //
    //   min_x, max_y     max_x, max_y
    //     +-----------------------+
    //     |   A               B   |
    //     |                       |
    //     |          C            |
    //     |                D      |
    //     |                       |
    //     +-----------------------+
    //   min_x, min_y     max_x, min_y

    let min_x = coordinates.iter().min_by_key(|a| a.x).unwrap().x;
    let max_x = coordinates.iter().max_by_key(|a| a.x).unwrap().x;

    let min_y = coordinates.iter().min_by_key(|a| a.y).unwrap().y;
    let max_y = coordinates.iter().max_by_key(|a| a.y).unwrap().y;

    // Step through every possible position
    for x in min_x..max_x + 1 {
        for y in min_y..max_y + 1 {
            let mut distance = Vec::with_capacity(coordinates.len());
            // For each position calculate manhattan distance for each point
            for (i, coord) in coordinates.iter().enumerate() {
                distance.push((coord.manhattan_distance(&Point { x, y }), i));
            }
            // sort the vector with lowest first
            distance.sort();

            // Compare lowest and second lowest, it the same don't count
            // We just want the shortest one and skip the once which is eq
            if distance[0].0 != distance[1].0 {
                if x == min_x || x == max_x || y == min_y || y == max_y {
                    area_count[distance[0].1] = isize::min_value();
                } else {
                    area_count[distance[0].1] += 1;
                }
            }
        }
    }
    *area_count.iter().max().unwrap()
}

fn part2() -> isize {
    let coordinates: Vec<Point> = get_coordinates();
    let min_x = coordinates.iter().min_by_key(|a| a.x).unwrap().x;
    let max_x = coordinates.iter().max_by_key(|a| a.x).unwrap().x;

    let min_y = coordinates.iter().min_by_key(|a| a.y).unwrap().y;
    let max_y = coordinates.iter().max_by_key(|a| a.y).unwrap().y;

    let mut area_count = 0;
    for x in min_x..max_x + 1 {
        for y in min_y..max_y + 1 {
            let mut total_distance = 0;
            for coord in &coordinates {
                total_distance += coord.manhattan_distance(&Point { x, y });
            }
            if total_distance < 10000 {
                area_count += 1;
            }
        }
    }
    area_count
}
fn main() {
    println!("Results for Day 6");
    println!("-------------------");

    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manhattan_distance() {
        assert_eq!(
            Point { x: 0, y: 0 }.manhattan_distance(&Point { x: 1, y: 1 }),
            2
        );
        assert_eq!(
            Point { x: -1, y: 0 }.manhattan_distance(&Point { x: 1, y: 1 }),
            3
        );
        assert_eq!(
            Point { x: -1, y: -1 }.manhattan_distance(&Point { x: -2, y: -1 }),
            1
        );
    }
}
