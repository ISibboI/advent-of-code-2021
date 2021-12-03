use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn problem_1() {
    #[derive(Default)]
    struct Counts {
        counts: Vec<usize>,
        total: usize,
    }

    let input = BufReader::new(File::open("inputs/3.txt").unwrap());
    let counts = input.lines().fold(Counts::default(), |mut counts, line| {
        let line = line.unwrap();
        let bits = line.as_str();
        if counts.total == 0 {
            counts.counts = vec![0; bits.len()];
        }
        assert_eq!(counts.counts.len(), bits.len());
        counts.total += 1;

        for (bit, count) in bits.chars().zip(counts.counts.iter_mut()) {
            if bit == '1' {
                *count += 1;
            }
        }

        counts
    });

    let mut number_1 = 0;
    let mut number_2 = 0;
    for (index, count) in counts.counts.iter().rev().enumerate() {
        let exponent = 1 << index;
        if *count > counts.total - *count {
            number_1 += exponent;
        } else if *count < counts.total - *count {
            number_2 += exponent;
        } else {
            panic!("Equal amount of zeroes and ones");
        }
    }

    let result = number_1 * number_2;

    println!("Day 3 problem 1: {}", result);
}

pub fn problem_2() {
    let lines: Vec<_> = BufReader::new(File::open("inputs/3.txt").unwrap())
        .lines()
        .map(Result::unwrap)
        .collect();
    let mut oxygen_lines: Vec<_> = lines.iter().map(String::as_str).collect();
    let mut scrubber_lines: Vec<_> = lines.iter().map(String::as_str).collect();

    for bit_index in 0.. {
        let mut partition_0 = Vec::new();
        let mut partition_1 = Vec::new();

        for line in oxygen_lines {
            if line.chars().nth(bit_index).unwrap() == '0' {
                partition_0.push(line);
            } else if line.chars().nth(bit_index).unwrap() == '1' {
                partition_1.push(line);
            } else {
                panic!("Unknown bit");
            }
        }

        if partition_1.len() >= partition_0.len() {
            oxygen_lines = partition_1;
        } else {
            oxygen_lines = partition_0;
        }

        if oxygen_lines.len() == 1 {
            break;
        }
    }

    for bit_index in 0usize.. {
        let mut partition_0 = Vec::new();
        let mut partition_1 = Vec::new();

        for line in scrubber_lines {
            if line.chars().nth(bit_index).unwrap() == '0' {
                partition_0.push(line);
            } else if line.chars().nth(bit_index).unwrap() == '1' {
                partition_1.push(line);
            } else {
                panic!("Unknown bit");
            }
        }

        if partition_1.len() < partition_0.len() {
            scrubber_lines = partition_1;
        } else {
            scrubber_lines = partition_0;
        }

        if scrubber_lines.len() == 1 {
            break;
        }
    }

    let result: usize = usize::from_str_radix(oxygen_lines[0], 2).unwrap()
        * usize::from_str_radix(scrubber_lines[0], 2).unwrap();

    println!("Day 3 problem 2: {}", result);
}
