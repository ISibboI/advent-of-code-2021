use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn problem_1() {
    let input = BufReader::new(File::open("inputs/1.txt").unwrap());
    let input: Vec<u16> = input.lines().map(|s| s.unwrap().parse().unwrap()).collect();
    let result: usize = input
        .windows(2)
        .map(|w| if w[0] < w[1] { 1 } else { 0 })
        .sum();
    println!("Day 1 problem 1: {}", result);
}

pub fn problem_2() {
    let input = BufReader::new(File::open("inputs/1.txt").unwrap());
    let input: Vec<u16> = input.lines().map(|s| s.unwrap().parse().unwrap()).collect();
    let result: usize = input
        .windows(4)
        .map(|w| if w[0] < w[3] { 1 } else { 0 })
        .sum();
    println!("Day 1 problem 2: {}", result);
}
