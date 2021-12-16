use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn problem_1() {
    let mut lines = BufReader::new(File::open("inputs/14.txt").unwrap()).lines();
    let mut string = lines.next().unwrap().unwrap();
    lines.next();
    let rules: HashMap<_, _> = lines
        .map(|line| {
            let line = line.unwrap();
            let pair = line.split_once(" -> ").unwrap();
            let key = (
                pair.0.chars().nth(0).unwrap(),
                pair.0.chars().nth(1).unwrap(),
            );
            (key, pair.1.chars().nth(0).unwrap())
        })
        .collect();

    for _ in 0..10 {
        let mut new_string = String::new();

        for ((_, char_1), (_, char_2)) in string.char_indices().zip(string.char_indices().skip(1)) {
            new_string.push(char_1);
            if let Some(&value) = rules.get(&(char_1, char_2)) {
                new_string.push(value);
            }
        }

        new_string.push(string.chars().rev().nth(0).unwrap());
        string = new_string;
    }

    let counts = string.chars().fold(HashMap::new(), |mut counts, char| {
        if let Some(count) = counts.get_mut(&char) {
            *count += 1;
        } else {
            counts.insert(char, 1);
        }
        counts
    });

    let max = counts.values().max().unwrap();
    let min = counts.values().min().unwrap();

    println!("Day 14 problem 1: {}", max - min);
}

pub fn problem_2() {
    let mut lines = BufReader::new(File::open("inputs/14.txt").unwrap()).lines();
    let string = lines.next().unwrap().unwrap();
    lines.next();
    let rules: HashMap<_, _> = lines
        .map(|line| {
            let line = line.unwrap();
            let pair = line.split_once(" -> ").unwrap();
            let key = (
                pair.0.chars().nth(0).unwrap(),
                pair.0.chars().nth(1).unwrap(),
            );
            (key, pair.1.chars().nth(0).unwrap())
        })
        .collect();

    let mut counts = HashMap::new();
    for offset in 0..(string.chars().count() - 1) {
        let char_1 = string.chars().nth(offset).unwrap();
        let char_2 = string.chars().nth(offset + 1).unwrap();
        if let Some(count) = counts.get_mut(&(char_1, char_2)) {
            *count += 1;
        } else {
            counts.insert((char_1, char_2), 1usize);
        }
    }

    for _ in 0..40 {
        let mut new_counts = HashMap::new();

        for ((char_1, char_2), count) in counts {
            if let Some(&value) = rules.get(&(char_1, char_2)) {
                if let Some(new_count) = new_counts.get_mut(&(char_1, value)) {
                    *new_count += count;
                } else {
                    new_counts.insert((char_1, value), count);
                }

                if let Some(new_count) = new_counts.get_mut(&(value, char_2)) {
                    *new_count += count;
                } else {
                    new_counts.insert((value, char_2), count);
                }
            } else if let Some(new_count) = new_counts.get_mut(&(char_1, char_2)) {
                *new_count += count;
            } else {
                new_counts.insert((char_1, char_2), count);
            }
        }

        counts = new_counts;
    }

    let single_counts = counts.into_iter().fold(
        HashMap::new(),
        |mut single_counts, ((char_1, char_2), count)| {
            if let Some(single_count) = single_counts.get_mut(&char_1) {
                *single_count += count;
            } else {
                single_counts.insert(char_1, count);
            }
            if let Some(single_count) = single_counts.get_mut(&char_2) {
                *single_count += count;
            } else {
                single_counts.insert(char_2, count);
            }
            single_counts
        },
    );

    let max = single_counts.values().max().unwrap();
    let min = single_counts.values().min().unwrap();

    println!("Day 14 problem 2: {}", (max - min) / 2 + 1);
}
