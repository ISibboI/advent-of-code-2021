use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

pub struct BingoField {
    field: Vec<bool>,
    numbers: BTreeMap<usize, usize>,
}

impl BingoField {
    pub fn new(numbers: Vec<usize>) -> Self {
        Self {
            field: vec![false; numbers.len()],
            numbers: numbers.iter().enumerate().map(|(i, &n)| (n, i)).collect(),
        }
    }

    pub fn set_number(&mut self, number: usize) {
        if let Some(&index) = self.numbers.get(&number) {
            *self.field.get_mut(index).unwrap() = true;
        }
    }

    pub fn is_bingo(&self) -> bool {
        for i in 0..5 {
            let mut x_bingo = true;
            let mut y_bingo = true;

            for j in 0..5 {
                if !self.field[i + 5 * j] {
                    x_bingo = false;
                }
                if !self.field[i * 5 + j] {
                    y_bingo = false;
                }
            }

            if x_bingo || y_bingo {
                return true;
            }
        }

        false
    }

    pub fn sum_not_marked(&self) -> usize {
        let mut sum = 0;
        for (k, v) in &self.numbers {
            if !self.field[*v] {
                sum += *k;
            }
        }
        sum
    }
}

pub fn problem_1() {
    let lines: Vec<_> = BufReader::new(File::open("inputs/4.txt").unwrap())
        .lines()
        .map(Result::unwrap)
        .collect();

    let numbers: Vec<_> = lines[0]
        .split(",")
        .map(usize::from_str)
        .map(Result::unwrap)
        .collect();
    let mut puzzles: Vec<_> = lines[1..]
        .chunks(6)
        .map(|chunk| {
            let chunk = &chunk[1..];
            let mut numbers = Vec::new();
            for line in chunk {
                for number in line.split(" ") {
                    if number.len() == 0 {
                        continue;
                    }

                    let number = number.trim().parse().unwrap();
                    numbers.push(number);
                }
            }
            BingoField::new(numbers)
        })
        .collect();

    for number in numbers {
        for board in &mut puzzles {
            board.set_number(number);
            if board.is_bingo() {
                let result = number * board.sum_not_marked();
                println!("Day 4 problem 1: {}", result);
                return;
            }
        }
    }
}

pub fn problem_2() {
    let lines: Vec<_> = BufReader::new(File::open("inputs/4.txt").unwrap())
        .lines()
        .map(Result::unwrap)
        .collect();

    let numbers: Vec<_> = lines[0]
        .split(",")
        .map(usize::from_str)
        .map(Result::unwrap)
        .collect();
    let mut puzzles: Vec<_> = lines[1..]
        .chunks(6)
        .map(|chunk| {
            let chunk = &chunk[1..];
            let mut numbers = Vec::new();
            for line in chunk {
                for number in line.split(" ") {
                    if number.len() == 0 {
                        continue;
                    }

                    let number = number.trim().parse().unwrap();
                    numbers.push(number);
                }
            }
            BingoField::new(numbers)
        })
        .collect();

    let mut last_result = 0;
    let mut has_won = vec![false; puzzles.len()];
    for number in numbers {
        for (board_index, board) in puzzles.iter_mut().enumerate() {
            board.set_number(number);
            if board.is_bingo() && !has_won[board_index] {
                has_won[board_index] = true;
                last_result = number * board.sum_not_marked();
            }
        }
    }

    println!("Day 4 problem 2: {}", last_result);
}
