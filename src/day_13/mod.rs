use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn fold(&self, fold: &Fold) -> Self {
        match fold {
            Fold::X(fold) => {
                if self.x > *fold {
                    Self {
                        x: 2 * *fold - self.x,
                        y: self.y,
                    }
                } else {
                    self.clone()
                }
            }
            Fold::Y(fold) => {
                if self.y > *fold {
                    Self {
                        y: 2 * *fold - self.y,
                        x: self.x,
                    }
                } else {
                    self.clone()
                }
            }
        }
    }
}

enum Fold {
    X(i32),
    Y(i32),
}

pub fn problem_1() {
    let mut points = HashSet::new();
    let mut folds = Vec::new();

    for line in BufReader::new(File::open("inputs/13.txt").unwrap()).lines() {
        let line = line.unwrap();
        if line.is_empty() {
            continue;
        }

        if line.contains(',') {
            let mut coordinates = line.split(',').map(i32::from_str).map(Result::unwrap);
            points.insert(Point {
                x: coordinates.next().unwrap(),
                y: coordinates.next().unwrap(),
            });
        } else {
            let line = &line[11..];
            folds.push(if line.starts_with('x') {
                Fold::X(line[2..].parse().unwrap())
            } else {
                Fold::Y(line[2..].parse().unwrap())
            });
        }
    }

    let points: HashSet<_> = points.iter().map(|p| p.fold(&folds[0])).collect();

    println!("Day 13 problem 1: {}", points.len());
}

pub fn problem_2() {
    let mut points = HashSet::new();
    let mut folds = Vec::new();

    for line in BufReader::new(File::open("inputs/13.txt").unwrap()).lines() {
        let line = line.unwrap();
        if line.is_empty() {
            continue;
        }

        if line.contains(',') {
            let mut coordinates = line.split(',').map(i32::from_str).map(Result::unwrap);
            points.insert(Point {
                x: coordinates.next().unwrap(),
                y: coordinates.next().unwrap(),
            });
        } else {
            let line = &line[11..];
            folds.push(if line.starts_with('x') {
                Fold::X(line[2..].parse().unwrap())
            } else {
                Fold::Y(line[2..].parse().unwrap())
            });
        }
    }

    for fold in folds {
        let new_points: HashSet<_> = points.iter().map(|p| p.fold(&fold)).collect();
        points = new_points;
    }

    let min_x = points.iter().map(|p| p.x).min().unwrap();
    let min_y = points.iter().map(|p| p.y).min().unwrap();
    let max_x = points.iter().map(|p| p.x).max().unwrap();
    let max_y = points.iter().map(|p| p.y).max().unwrap();

    println!("Day 13 problem 2:");
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if points.contains(&Point { x, y }) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}
