use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

pub fn problem_1() {
    fn costs(positions: &[i32], target: i32) -> i32 {
        positions.iter().map(|x| (x - target).abs()).sum()
    }

    let positions: Vec<_> = BufReader::new(File::open("inputs/7.txt").unwrap())
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(i32::from_str)
        .map(Result::unwrap)
        .collect();
    let guess = positions.iter().sum::<i32>() / positions.len() as i32;

    let direction = match costs(&positions, guess).cmp(&costs(&positions, guess + 1)) {
        Ordering::Less => -1,
        Ordering::Greater => 1,
        Ordering::Equal => 1, // Should be at min in that case
    };

    let mut result = guess;
    loop {
        let new_guess = result + direction;
        if costs(&positions, new_guess) > costs(&positions, result) {
            break;
        } else {
            result = new_guess;
        }
    }

    println!("Day 7 problem 1: {}", costs(&positions, result));
}

pub fn problem_2() {
    fn costs(positions: &[i32], target: i32) -> i32 {
        positions
            .iter()
            .map(|x| (x - target).abs())
            .map(|n| n * (n + 1) / 2)
            .sum()
    }

    let positions: Vec<_> = BufReader::new(File::open("inputs/7.txt").unwrap())
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(i32::from_str)
        .map(Result::unwrap)
        .collect();
    let guess = positions.iter().sum::<i32>() / positions.len() as i32;

    let direction = match costs(&positions, guess).cmp(&costs(&positions, guess + 1)) {
        Ordering::Less => -1,
        Ordering::Greater => 1,
        Ordering::Equal => 1, // Should be at min in that case
    };

    let mut result = guess;
    loop {
        let new_guess = result + direction;
        if costs(&positions, new_guess) > costs(&positions, result) {
            break;
        } else {
            result = new_guess;
        }
    }

    println!("Day 7 problem 2: {}", costs(&positions, result));
}
