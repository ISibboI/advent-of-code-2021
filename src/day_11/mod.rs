use std::fs::File;
use std::io::{BufRead, BufReader};

fn neighbors(pos: usize) -> Vec<usize> {
    let mut result = Vec::new();

    if pos >= 10 {
        result.push(pos - 10);

        if pos % 10 > 0 {
            result.push(pos - 11);
        }

        if pos % 10 < 9 {
            result.push(pos - 9);
        }
    }

    if pos % 10 > 0 {
        result.push(pos - 1);
    }

    if pos % 10 < 9 {
        result.push(pos + 1);
    }

    if pos < 90 {
        result.push(pos + 10);

        if pos % 10 > 0 {
            result.push(pos + 9);
        }

        if pos % 10 < 9 {
            result.push(pos + 11);
        }
    }

    result
}

pub fn problem_1() {
    let mut grid: Vec<u32> = BufReader::new(File::open("inputs/11.txt").unwrap())
        .lines()
        .map(|line| {
            let line = line.unwrap();
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect();

    let mut result = 0;
    for _ in 0..100 {
        for item in &mut grid {
            *item += 1;
        }

        loop {
            let previous_result = result;
            for pos in 0..grid.len() {
                if grid[pos] > 9 {
                    grid[pos] = 0;
                    result += 1;

                    for neighbor in neighbors(pos) {
                        if grid[neighbor] > 0 {
                            grid[neighbor] += 1;
                        }
                    }
                }
            }

            if result == previous_result {
                break;
            }
        }
    }

    println!("Day 11 problem 1: {}", result);
}

pub fn problem_2() {
    let mut grid: Vec<u32> = BufReader::new(File::open("inputs/11.txt").unwrap())
        .lines()
        .map(|line| {
            let line = line.unwrap();
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect();

    let mut result = 1;
    loop {
        for item in &mut grid {
            *item += 1;
        }

        let mut flashes = 0;
        loop {
            let previous_flashes = flashes;
            for pos in 0..grid.len() {
                if grid[pos] > 9 {
                    grid[pos] = 0;
                    flashes += 1;

                    for neighbor in neighbors(pos) {
                        if grid[neighbor] > 0 {
                            grid[neighbor] += 1;
                        }
                    }
                }
            }

            if flashes == previous_flashes {
                break;
            }
        }

        if flashes == 100 {
            break;
        }
        result += 1;
    }

    println!("Day 11 problem 2: {}", result);
}
