use std::collections::BTreeSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub struct Line {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
}

fn interval_overlap(a1: i32, b1: i32, a2: i32, b2: i32) -> Option<(i32, i32)> {
    let l1 = a1.min(b1);
    let r1 = a1.max(b1);
    let l2 = a2.min(b2);
    let r2 = a2.max(b2);

    let left = l1.max(l2);
    let right = r1.min(r2);
    if left <= right {
        Some((left, right))
    } else {
        None
    }
}

impl Line {
    pub fn new(x1: i32, x2: i32, y1: i32, y2: i32) -> Self {
        assert!(!x1.is_negative());
        assert!(!x2.is_negative());
        assert!(!y1.is_negative());
        assert!(!y2.is_negative());
        Self { x1, x2, y1, y2 }
    }

    pub fn points(&self) -> Vec<(i32, i32)> {
        match self.m() {
            Some(0) => (self.minx()..=self.maxx()).map(|x| (x, self.y1)).collect(),
            None => (self.miny()..=self.maxy()).map(|y| (self.x1, y)).collect(),
            Some(1) => (self.minx()..=self.maxx())
                .zip(self.miny()..=self.maxy())
                .collect(),
            Some(-1) => (self.minx()..=self.maxx())
                .zip((self.miny()..=self.maxy()).rev())
                .collect(),
            _ => panic!(),
        }
    }

    pub fn intersection(&self, other: &Self) -> Vec<(i32, i32)> {
        if self.m() != Some(0) && self.m() != None {
            return Vec::new();
        }
        if other.m() != Some(0) && other.m() != None {
            return Vec::new();
        }

        if self.m() == other.m() {
            if self.dx() == 0 && self.x1 == other.x1 {
                assert_eq!(other.dx(), 0);
                //println!("self inf other inf");
                if let Some((y1, y2)) = interval_overlap(self.y1, self.y2, other.y1, other.y2) {
                    (y1..=y2).map(|y| (self.x1, y)).collect()
                } else {
                    Vec::new()
                }
            } else if self.dy() == 0 && self.y1 == other.y1 {
                assert_eq!(other.dy(), 0);
                //println!("self 0 other 0");
                if let Some((x1, x2)) = interval_overlap(self.x1, self.x2, other.x1, other.x2) {
                    (x1..=x2).map(|x| (x, self.y1)).collect()
                } else {
                    Vec::new()
                }
            } else {
                Vec::new()
            }
        } else {
            if self.m() == Some(0)
                && self.minx() <= other.x1
                && other.x1 <= self.maxx()
                && other.miny() <= self.y1
                && self.y1 <= other.maxy()
            {
                assert_eq!(other.m(), None);
                assert_eq!(self.m(), Some(0));
                //println!("self 0 other inf");
                vec![(other.x1, self.y1)]
            } else if other.m() == Some(0)
                && other.minx() <= self.x1
                && self.x1 <= other.maxx()
                && self.miny() <= other.y1
                && other.y1 <= self.maxy()
            {
                assert_eq!(self.m(), None);
                assert_eq!(other.m(), Some(0));
                //println!("self inf other 0");
                vec![(self.x1, other.y1)]
            } else {
                Vec::new()
            }
        }
    }

    pub fn m(&self) -> Option<i32> {
        self.dy().checked_div(self.dx())
    }

    pub fn dx(&self) -> i32 {
        self.x2 - self.x1
    }

    pub fn dy(&self) -> i32 {
        self.y2 - self.y1
    }

    pub fn minx(&self) -> i32 {
        self.x1.min(self.x2)
    }

    pub fn miny(&self) -> i32 {
        self.y1.min(self.y2)
    }

    pub fn maxx(&self) -> i32 {
        self.x1.max(self.x2)
    }

    pub fn maxy(&self) -> i32 {
        self.y1.max(self.y2)
    }

    pub fn is_on_line(&self, x: i32, y: i32) -> bool {
        if self.m() == Some(0) || self.m() == None {
            self.minx() <= x && self.maxx() >= x && self.miny() <= y && self.maxy() >= y
        } else if self.m() == Some(1) {
            self.minx() <= x
                && self.maxx() >= x
                && self.miny() <= y
                && self.maxy() >= y
                && x - self.minx() == y - self.miny()
        } else if self.m() == Some(-1) {
            self.minx() <= x
                && self.maxx() >= x
                && self.miny() <= y
                && self.maxy() >= y
                && x - self.minx() == self.maxy() - y
        } else {
            panic!()
        }
    }
}

pub fn problem_1() {
    let lines: Vec<_> = BufReader::new(File::open("inputs/5.txt").unwrap())
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let mut points = line.split(" -> ");
            let mut p1 = points.next().unwrap().split(',');
            let x1 = p1.next().unwrap().parse().unwrap();
            let y1 = p1.next().unwrap().parse().unwrap();
            let mut p2 = points.next().unwrap().split(',');
            let x2 = p2.next().unwrap().parse().unwrap();
            let y2 = p2.next().unwrap().parse().unwrap();
            Line::new(x1, x2, y1, y2)
        })
        .collect();

    let mut intersections = BTreeSet::new();
    for (i, line1) in lines.iter().enumerate() {
        assert!(line1.dx() != 0 || line1.dy() != 0);
        for line2 in lines.iter().skip(i + 1) {
            for intersection in line1.intersection(line2) {
                assert!(line1.m() == Some(0) || line1.m() == None);
                assert!(line2.m() == Some(0) || line2.m() == None);
                assert!(
                    line1.is_on_line(intersection.0, intersection.1),
                    "l1: {:?}, l2: {:?}, intersection: {:?}",
                    line1,
                    line2,
                    intersection
                );
                assert!(
                    line2.is_on_line(intersection.0, intersection.1),
                    "l1: {:?}, l2: {:?}, intersection: {:?}",
                    line1,
                    line2,
                    intersection
                );
                intersections.insert(intersection);
            }
        }
    }

    println!("Day 5 problem 1: {}", intersections.len());
}

pub fn problem_2() {
    let lines: Vec<_> = BufReader::new(File::open("inputs/5.txt").unwrap())
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let mut points = line.split(" -> ");
            let mut p1 = points.next().unwrap().split(',');
            let x1 = p1.next().unwrap().parse().unwrap();
            let y1 = p1.next().unwrap().parse().unwrap();
            let mut p2 = points.next().unwrap().split(',');
            let x2 = p2.next().unwrap().parse().unwrap();
            let y2 = p2.next().unwrap().parse().unwrap();
            Line::new(x1, x2, y1, y2)
        })
        .collect();

    let max_x = lines.iter().map(|l| l.x1.max(l.x2)).max().unwrap();
    let max_y = lines.iter().map(|l| l.y1.max(l.y2)).max().unwrap();
    assert!(max_x < 1000);
    assert!(max_y < 1000);

    let mut map = vec![[0; 1000]; 1000];

    for line in &lines {
        assert!(
            line.m() == Some(-1) || line.m() == Some(0) || line.m() == Some(1) || line.m() == None
        );
        for point in line.points() {
            assert!(
                line.is_on_line(point.0, point.1),
                "line: {:?}, m: {:?}, point: {:?}",
                &line,
                line.m(),
                point
            );
            assert!(point.0 >= 0);
            assert!(point.1 >= 0);
            map[point.0 as usize][point.1 as usize] += 1;
        }
    }

    let mut result = 0;
    for t in map.iter() {
        for n in t.iter() {
            if *n >= 2 {
                result += 1;
            }
        }
    }

    println!("Day 5 problem 2: {}", result);
}
