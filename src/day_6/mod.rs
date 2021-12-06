use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

pub fn problem_1() {
    let mut ages: Vec<_> = BufReader::new(File::open("inputs/6.txt").unwrap())
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(u8::from_str)
        .map(Result::unwrap)
        .collect();
    for _ in 0..80 {
        let limit = ages.len();

        for i in 0..limit {
            if ages[i] == 0 {
                ages[i] = 6;
                ages.push(8);
            } else {
                ages[i] -= 1;
            }
        }
    }

    println!("Day 6 problem 1: {}", ages.len());
}

pub fn problem_2() {
    let mut ages = vec![0u64; 9];
    BufReader::new(File::open("inputs/6.txt").unwrap())
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(usize::from_str)
        .map(Result::unwrap)
        .for_each(|age| {
            ages[age] += 1;
        });
    for _ in 0..256 {
        let mut new_ages = vec![0u64; 9];

        for age in 0..8 {
            new_ages[age] = ages[age + 1];
        }

        new_ages[8] = ages[0];
        new_ages[6] += ages[0];
        ages = new_ages;
    }

    println!("Day 6 problem 2: {}", ages.iter().sum::<u64>());
}
