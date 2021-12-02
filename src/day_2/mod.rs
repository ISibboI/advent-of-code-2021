use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn problem_1() {
    #[derive(Default)]
    struct Position {
        pub forward: u32,
        pub depth: u32,
    }

    let input = BufReader::new(File::open("inputs/2.txt").unwrap());
    let result = input.lines().fold(Position::default(), |mut position, line| {
        let columns: Vec<_> = line.unwrap().split(" ").map(ToOwned::to_owned).collect();
        let value: u32 = columns[1].parse().unwrap();
        match columns[0].as_str() {
            "forward" => position.forward += value,
            "up" => position.depth -= value,
            "down" => position.depth += value,
            _ => panic!("Unknown command: {}", columns[0]),
        }
        position
    });
    let result = result.depth * result.forward;

    println!("Day 2 problem 1: {}", result);
}

pub fn problem_2() {
    #[derive(Default)]
    struct Position {
        pub forward: u32,
        pub depth: u32,
        pub aim: u32,
    }

    let input = BufReader::new(File::open("inputs/2.txt").unwrap());
    let result = input.lines().fold(Position::default(), |mut position, line| {
        let columns: Vec<_> = line.unwrap().split(" ").map(ToOwned::to_owned).collect();
        let value: u32 = columns[1].parse().unwrap();
        match columns[0].as_str() {
            "forward" => {position.forward += value; position.depth += position.aim * value}
            "up" => position.aim -= value,
            "down" => position.aim += value,
            _ => panic!("Unknown command: {}", columns[0]),
        }
        position
    });
    let result = result.depth * result.forward;

    println!("Day 2 problem 1: {}", result);
}