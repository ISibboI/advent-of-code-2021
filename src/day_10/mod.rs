use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn problem_1() {
    let result: u64 = BufReader::new(File::open("inputs/10.txt").unwrap())
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let mut stack = Vec::new();

            let mut result = 0;
            for character in line.chars() {
                match character {
                    '(' => stack.push(')'),
                    '[' => stack.push(']'),
                    '{' => stack.push('}'),
                    '<' => stack.push('>'),
                    ')' | ']' | '}' | '>' => {
                        if let Some(top) = stack.pop() {
                            if top != character {
                                result = match character {
                                    ')' => 3,
                                    ']' => 57,
                                    '}' => 1197,
                                    '>' => 25137,
                                    _ => unreachable!(),
                                };
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                    _ => panic!(),
                }
            }

            result
        })
        .sum();

    println!("Day 10 problem 1: {}", result);
}

pub fn problem_2() {
    let mut scores: Vec<_> = BufReader::new(File::open("inputs/10.txt").unwrap())
        .lines()
        .filter_map(|line| {
            let line = line.unwrap();
            let mut stack = Vec::new();

            for character in line.chars() {
                match character {
                    '(' => stack.push(')'),
                    '[' => stack.push(']'),
                    '{' => stack.push('}'),
                    '<' => stack.push('>'),
                    ')' | ']' | '}' | '>' => {
                        if let Some(top) = stack.pop() {
                            if top != character {
                                return None;
                            }
                        } else {
                            return None;
                        }
                    }
                    _ => panic!(),
                }
            }

            if stack.is_empty() {
                return None;
            }

            let mut result: u64 = 0;
            while let Some(character) = stack.pop() {
                result *= 5;
                result += match character {
                    ')' => 1,
                    ']' => 2,
                    '}' => 3,
                    '>' => 4,
                    _ => unreachable!(),
                };
            }
            assert!(result > 0);
            Some(result)
        })
        .collect();

    scores.sort_unstable();
    let result = scores[scores.len() / 2];

    // too low: 617930596
    println!("Day 10 problem 2: {}", result);
}
