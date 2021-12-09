use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn neighbors(&self) -> Vec<Self> {
        vec![
            Self::new(self.x, self.y + 1),
            Self::new(self.x, self.y - 1),
            Self::new(self.x + 1, self.y),
            Self::new(self.x - 1, self.y),
        ]
    }
}

pub fn problem_1() {
    let points: HashMap<_, _> = BufReader::new(File::open("inputs/9.txt").unwrap())
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.unwrap()
                .chars()
                .enumerate()
                .map(|(x, height)| (Point::new(x as i32, y as i32), height.to_digit(10).unwrap()))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect();

    let mut result = 0;
    for (point, height) in &points {
        let mut is_minimum = true;
        for neighbor in point.neighbors() {
            if let Some(neighbor_height) = points.get(&neighbor) {
                if neighbor_height <= height {
                    is_minimum = false;
                    break;
                }
            }
        }

        if is_minimum {
            result += 1 + height;
        }
    }

    println!("Day 9 problem 1: {}", result);
}

pub fn problem_2() {
    let points: HashMap<_, _> = BufReader::new(File::open("inputs/9.txt").unwrap())
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.unwrap()
                .chars()
                .enumerate()
                .map(|(x, height)| (Point::new(x as i32, y as i32), height.to_digit(10).unwrap()))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect();

    let mut low_points = HashSet::new();
    for (point, height) in &points {
        let mut is_minimum = true;
        for neighbor in point.neighbors() {
            if let Some(neighbor_height) = points.get(&neighbor) {
                if neighbor_height <= height {
                    is_minimum = false;
                    break;
                }
            }
        }

        if is_minimum {
            low_points.insert(point.clone());
        }
    }

    let mut closed_points = HashSet::new();
    let mut sizes = Vec::new();
    for low_point in low_points {
        assert!(!closed_points.contains(&low_point));
        let mut queue = HashSet::new();
        queue.insert(low_point);
        let mut size = 0;

        while let Some(current) = queue.iter().cloned().next() {
            queue.remove(&current);
            assert!(!closed_points.contains(&current));
            closed_points.insert(current.clone());
            size += 1;
            let &current_height = points.get(&current).unwrap();

            for neighbor in current.neighbors() {
                if closed_points.contains(&neighbor) || queue.contains(&neighbor) {
                    continue;
                }

                if let Some(&neighbor_height) = points.get(&neighbor) {
                    if neighbor_height > current_height && neighbor_height < 9 {
                        queue.insert(neighbor);
                    }
                }
            }
        }

        sizes.push(size);
    }

    sizes.sort_unstable();
    let result = sizes.iter().rev().take(3).fold(1, |acc, n| acc * n);

    println!("Day 9 problem 2: {}", result);
}
