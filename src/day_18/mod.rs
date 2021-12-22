use std::borrow::Borrow;
use std::cell::RefCell;
use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::rc::Rc;

#[derive(Clone)]
enum SnailFishSuccessor {
    Number(u32),
    SnailFishNumber(Rc<RefCell<SnailFishNumber>>),
}

#[derive(Clone)]
struct SnailFishNumber {
    pub parent: Option<Rc<RefCell<SnailFishNumber>>>,
    pub left: SnailFishSuccessor,
    pub right: SnailFishSuccessor,
}

impl Debug for SnailFishNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        match &self.left {
            SnailFishSuccessor::Number(number) => write!(f, "{}", number)?,
            SnailFishSuccessor::SnailFishNumber(number) => {
                write!(f, "{:?}", number.as_ref().borrow())?
            }
        }
        write!(f, ",")?;
        match &self.right {
            SnailFishSuccessor::Number(number) => write!(f, "{}", number)?,
            SnailFishSuccessor::SnailFishNumber(number) => {
                write!(f, "{:?}", number.as_ref().borrow())?
            }
        }
        write!(f, "]")
    }
}

impl SnailFishNumber {
    pub fn parse(mut string: &str) -> (&str, Rc<RefCell<Self>>) {
        assert_eq!(&string[0..=0], "[");
        string = &string[1..];

        let left = if string.chars().nth(0).unwrap().is_digit(10) {
            let left = SnailFishSuccessor::Number(string[0..=0].parse().unwrap());
            string = &string[1..];
            left
        } else {
            let (new_string, left) = Self::parse(string);
            string = new_string;
            SnailFishSuccessor::SnailFishNumber(left)
        };

        assert_eq!(&string[0..=0], ",");
        string = &string[1..];

        let right = if string.chars().nth(0).unwrap().is_digit(10) {
            let right = SnailFishSuccessor::Number(string[0..=0].parse().unwrap());
            string = &string[1..];
            right
        } else {
            let (new_string, right) = Self::parse(string);
            string = new_string;
            SnailFishSuccessor::SnailFishNumber(right)
        };

        assert_eq!(&string[0..=0], "]");
        string = &string[1..];

        let result = Rc::new(RefCell::new(Self {
            left,
            right,
            parent: None,
        }));

        let result_borrow: &RefCell<SnailFishNumber> = result.borrow();
        if let SnailFishSuccessor::SnailFishNumber(left) = result_borrow.borrow().left.clone() {
            left.as_ref().borrow_mut().parent = Some(result.clone());
        }
        if let SnailFishSuccessor::SnailFishNumber(right) = result_borrow.borrow().right.clone() {
            right.as_ref().borrow_mut().parent = Some(result.clone());
        }

        (string, result)
    }

    pub fn add(a: Rc<RefCell<Self>>, b: Rc<RefCell<Self>>) -> Rc<RefCell<Self>> {
        let result = Rc::new(RefCell::new(SnailFishNumber {
            parent: None,
            left: SnailFishSuccessor::SnailFishNumber(a.clone()),
            right: SnailFishSuccessor::SnailFishNumber(b.clone()),
        }));
        let result_borrow: &RefCell<SnailFishNumber> = result.borrow();
        if let SnailFishSuccessor::SnailFishNumber(left) = result_borrow.borrow().left.clone() {
            left.as_ref().borrow_mut().parent = Some(result.clone());
        }
        if let SnailFishSuccessor::SnailFishNumber(right) = result_borrow.borrow().right.clone() {
            right.as_ref().borrow_mut().parent = Some(result.clone());
        }

        fn find_explode(
            start: Rc<RefCell<SnailFishNumber>>,
            depth: u32,
        ) -> Option<Rc<RefCell<SnailFishNumber>>> {
            if depth >= 4 {
                let mut current = start;
                loop {
                    let left = {
                        let current_borrow: &RefCell<SnailFishNumber> = current.borrow();
                        current_borrow.borrow().left.clone()
                    };
                    if let SnailFishSuccessor::SnailFishNumber(left) = left {
                        current = left;
                    } else {
                        break;
                    }
                }

                let current_borrow: &RefCell<SnailFishNumber> = current.borrow();
                if let SnailFishSuccessor::SnailFishNumber(_) =
                    current_borrow.borrow().right.clone()
                {
                    panic!();
                }
                return Some(current);
            }

            let start_borrow: &RefCell<SnailFishNumber> = start.borrow();
            if let SnailFishSuccessor::SnailFishNumber(left) = start_borrow.borrow().left.clone() {
                if let Some(result) = find_explode(left, depth + 1) {
                    return Some(result);
                }
            }

            if let SnailFishSuccessor::SnailFishNumber(right) = start_borrow.borrow().right.clone()
            {
                if let Some(result) = find_explode(right, depth + 1) {
                    return Some(result);
                }
            }

            None
        }

        fn fix_split(start: Rc<RefCell<SnailFishNumber>>) -> Option<Rc<RefCell<SnailFishNumber>>> {
            let left_number = { start.as_ref().borrow().left.clone() };
            if let SnailFishSuccessor::SnailFishNumber(left) = left_number {
                if let Some(result) = fix_split(left) {
                    return Some(result);
                }
            } else {
                let mut start_mut = start.as_ref().borrow_mut();
                if let SnailFishSuccessor::Number(number) = &mut start_mut.left {
                    if *number > 9 {
                        let left = *number / 2;
                        let right = if left + left < *number {
                            left + 1
                        } else {
                            left
                        };

                        let result = Rc::new(RefCell::new(SnailFishNumber {
                            parent: Some(start.clone()),
                            left: SnailFishSuccessor::Number(left),
                            right: SnailFishSuccessor::Number(right),
                        }));
                        start_mut.left = SnailFishSuccessor::SnailFishNumber(result.clone());
                        return Some(result);
                    }
                }
            }

            let right_number = { start.as_ref().borrow().right.clone() };
            if let SnailFishSuccessor::SnailFishNumber(right) = right_number {
                if let Some(result) = fix_split(right) {
                    return Some(result);
                }
            } else {
                let mut start_mut = start.as_ref().borrow_mut();
                if let SnailFishSuccessor::Number(number) = &mut start_mut.right {
                    if *number > 9 {
                        let left = *number / 2;
                        let right = if left + left < *number {
                            left + 1
                        } else {
                            left
                        };

                        let result = Rc::new(RefCell::new(SnailFishNumber {
                            parent: Some(start.clone()),
                            left: SnailFishSuccessor::Number(left),
                            right: SnailFishSuccessor::Number(right),
                        }));
                        start_mut.right = SnailFishSuccessor::SnailFishNumber(result.clone());
                        return Some(result);
                    }
                }
            }

            None
        }

        loop {
            if let Some(explode) = find_explode(result.clone(), 0) {
                {
                    let explode_left = if let SnailFishSuccessor::Number(number) =
                        explode.as_ref().borrow().left
                    {
                        number
                    } else {
                        unreachable!()
                    };

                    // search upwards for predecessor
                    let mut current = explode.clone();
                    let mut found_left_branch = false;
                    loop {
                        let current_parent = {
                            let current_borrow: &RefCell<SnailFishNumber> = current.borrow();
                            current_borrow.borrow().parent.clone()
                        };
                        if let Some(current_parent) = current_parent {
                            let is_right_child = if let SnailFishSuccessor::SnailFishNumber(
                                current_parent_right_child,
                            ) =
                                current_parent.as_ref().borrow().right.clone()
                            {
                                current_parent_right_child.as_ptr() == current.as_ptr()
                            } else {
                                false
                            };

                            current = current_parent;
                            if is_right_child {
                                let new_current;
                                match &mut current.as_ref().borrow_mut().left {
                                    SnailFishSuccessor::Number(number) => {
                                        *number += explode_left;
                                        break;
                                    }
                                    SnailFishSuccessor::SnailFishNumber(number) => {
                                        new_current = number.clone()
                                    }
                                }
                                current = new_current;

                                found_left_branch = true;
                                break;
                            }
                        } else {
                            // No predecessor found
                            break;
                        }
                    }

                    // search downwards for predecessor
                    if found_left_branch {
                        loop {
                            let right = {
                                let current_borrow: &RefCell<SnailFishNumber> = current.borrow();
                                current_borrow.borrow().right.clone()
                            };
                            if let SnailFishSuccessor::SnailFishNumber(right) = right {
                                current = right;
                            } else {
                                break;
                            }
                        }

                        // found predecessor, increment
                        assert_ne!(current.as_ptr(), explode.as_ptr());
                        if let SnailFishSuccessor::Number(number) =
                            &mut current.as_ref().borrow_mut().right
                        {
                            *number += explode_left;
                        } else {
                            unreachable!()
                        }
                    }
                }

                {
                    let explode_right = if let SnailFishSuccessor::Number(number) =
                        explode.as_ref().borrow().right
                    {
                        number
                    } else {
                        unreachable!()
                    };

                    // search upwards for successor
                    let mut current = explode.clone();
                    let mut found_right_branch = false;
                    loop {
                        let current_parent = { current.as_ref().borrow().parent.clone() };
                        if let Some(current_parent) = current_parent {
                            let is_left_child = if let SnailFishSuccessor::SnailFishNumber(
                                current_parent_left_child,
                            ) = current_parent.as_ref().borrow().left.clone()
                            {
                                current_parent_left_child.as_ptr() == current.as_ptr()
                            } else {
                                false
                            };

                            current = current_parent;
                            if is_left_child {
                                let new_current;
                                match &mut current.as_ref().borrow_mut().right {
                                    SnailFishSuccessor::Number(number) => {
                                        *number += explode_right;
                                        break;
                                    }
                                    SnailFishSuccessor::SnailFishNumber(number) => {
                                        new_current = number.clone()
                                    }
                                }
                                current = new_current;

                                found_right_branch = true;
                                break;
                            }
                        } else {
                            // No successor found
                            break;
                        }
                    }

                    // search downwards for successor
                    if found_right_branch {
                        loop {
                            let left = {
                                let current_borrow: &RefCell<SnailFishNumber> = current.borrow();
                                current_borrow.borrow().left.clone()
                            };
                            if let SnailFishSuccessor::SnailFishNumber(left) = left {
                                current = left;
                            } else {
                                break;
                            }
                        }

                        // found successor, increment
                        assert_ne!(current.as_ptr(), explode.as_ptr());
                        if let SnailFishSuccessor::Number(number) =
                            &mut current.as_ref().borrow_mut().left
                        {
                            *number += explode_right;
                        } else {
                            unreachable!()
                        }
                    }
                }

                let parent = explode.as_ref().borrow_mut().parent.take().unwrap();
                let parent_borrow: &RefCell<SnailFishNumber> = parent.borrow();
                let is_left = if let SnailFishSuccessor::SnailFishNumber(parent_left) =
                    parent_borrow.borrow().left.clone()
                {
                    parent_left.as_ptr() == explode.as_ptr()
                } else {
                    false
                };
                if is_left {
                    parent.as_ref().borrow_mut().left = SnailFishSuccessor::Number(0);
                } else {
                    parent.as_ref().borrow_mut().right = SnailFishSuccessor::Number(0);
                }

                continue;
            }

            if let Some(_) = fix_split(result.clone()) {
                continue;
            }

            break;
        }

        result
    }

    pub fn magnitude(&self) -> u32 {
        let left = match &self.left {
            SnailFishSuccessor::SnailFishNumber(number) => number.as_ref().borrow().magnitude(),
            SnailFishSuccessor::Number(number) => *number,
        };
        let right = match &self.right {
            SnailFishSuccessor::SnailFishNumber(number) => number.as_ref().borrow().magnitude(),
            SnailFishSuccessor::Number(number) => *number,
        };
        3 * left + 2 * right
    }
}

pub fn problem_1() {
    let lines = BufReader::new(File::open("inputs/18.txt").unwrap())
        .lines()
        .map(Result::unwrap);

    let mut current: Option<Rc<RefCell<SnailFishNumber>>> = None;
    for line in lines {
        if let Some(current) = &mut current {
            let next = SnailFishNumber::parse(&line).1;
            *current = SnailFishNumber::add(current.clone(), next);
            println!("{:?}", current.as_ref().borrow());
        } else {
            current = Some(SnailFishNumber::parse(&line).1);
        }
    }

    println!(
        "Day 18 problem 1: {}",
        current.unwrap().as_ref().borrow().magnitude()
    );
}

pub fn problem_2() {
    let lines: Vec<_> = BufReader::new(File::open("inputs/18.txt").unwrap())
        .lines()
        .map(Result::unwrap)
        .collect();

    let mut max = 0;
    for (i, a) in lines.iter().enumerate() {
        for (j, b) in lines.iter().enumerate() {
            if lines[i] == lines[j] {
                continue;
            }
            let sum =
                SnailFishNumber::add(SnailFishNumber::parse(a).1, SnailFishNumber::parse(b).1)
                    .as_ref()
                    .borrow()
                    .clone();
            let magnitude = sum.magnitude();

            max = max.max(magnitude);
        }
    }

    // too high: 4757
    println!("Day 18 problem 2: {}", max);
}
