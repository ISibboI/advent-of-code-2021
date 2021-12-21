use std::fs::File;
use std::io::{BufRead, BufReader};
use traitgraph::traitsequence::interface::Sequence;

pub fn problem_1() {
    fn parse_packet(chars: &str, mut offset: usize) -> (usize, usize) {
        let version = usize::from_str_radix(&chars[offset..offset + 3], 2).unwrap();
        let type_id = usize::from_str_radix(&chars[offset + 3..offset + 6], 2).unwrap();
        offset += 6;

        if type_id == 4 {
            // literal value
            let mut number = String::new();
            let mut is_last = false;
            while !is_last {
                is_last = &chars[offset..=offset] == "0";
                number.push_str(&chars[offset + 1..offset + 5]);
                offset += 5;
            }

            let _number = usize::from_str_radix(&number, 2).unwrap();
            (offset, version)
        } else {
            // operator
            let length_type_id = usize::from_str_radix(&chars[offset..=offset], 2).unwrap();
            offset += 1;
            let mut result = version;

            if length_type_id == 0 {
                let total_length = usize::from_str_radix(&chars[offset..offset + 15], 2).unwrap();
                offset += 15;
                let original_offset = offset;

                loop {
                    let (new_offset, result_update) = parse_packet(chars, offset);
                    offset = new_offset;
                    result += result_update;

                    if offset - original_offset == total_length {
                        break;
                    } else if offset - original_offset > total_length {
                        panic!();
                    }
                }

                (offset, result)
            } else {
                // length_type_id == 1
                let subpacket_amount =
                    usize::from_str_radix(&chars[offset..offset + 11], 2).unwrap();
                offset += 11;

                for _ in 0..subpacket_amount {
                    let (new_offset, result_update) = parse_packet(chars, offset);
                    offset = new_offset;
                    result += result_update;
                }

                (offset, result)
            }
        }
    }

    let mut lines = BufReader::new(File::open("inputs/16.txt").unwrap()).lines();
    let line = lines.next().unwrap().unwrap();
    let mut binary = String::new();
    for c in line.chars() {
        binary.push_str(&format!("{:04b}", c.to_digit(16).unwrap()));
    }

    let (_parsed_char_count, result) = parse_packet(&binary, 0);

    println!("Day 16 problem 1: {}", result);
}

pub fn problem_2() {
    fn parse_packet(chars: &str, mut offset: usize) -> (usize, usize) {
        let _version = usize::from_str_radix(&chars[offset..offset + 3], 2).unwrap();
        let type_id = usize::from_str_radix(&chars[offset + 3..offset + 6], 2).unwrap();
        offset += 6;

        if type_id == 4 {
            // literal value
            let mut number = String::new();
            let mut is_last = false;
            while !is_last {
                is_last = &chars[offset..=offset] == "0";
                number.push_str(&chars[offset + 1..offset + 5]);
                offset += 5;
            }

            let number = usize::from_str_radix(&number, 2).unwrap();
            (offset, number)
        } else {
            // operator
            let length_type_id = usize::from_str_radix(&chars[offset..=offset], 2).unwrap();
            offset += 1;
            let mut operands = Vec::new();

            if length_type_id == 0 {
                let total_length = usize::from_str_radix(&chars[offset..offset + 15], 2).unwrap();
                offset += 15;
                let original_offset = offset;

                loop {
                    let (new_offset, operand) = parse_packet(chars, offset);
                    offset = new_offset;
                    operands.push(operand);

                    if offset - original_offset == total_length {
                        break;
                    } else if offset - original_offset > total_length {
                        panic!();
                    }
                }
            } else {
                // length_type_id == 1
                let subpacket_amount =
                    usize::from_str_radix(&chars[offset..offset + 11], 2).unwrap();
                offset += 11;

                for _ in 0..subpacket_amount {
                    let (new_offset, operand) = parse_packet(chars, offset);
                    offset = new_offset;
                    operands.push(operand);
                }
            }

            assert!(operands.len() > 0);
            if type_id >= 5 {
                assert_eq!(operands.len(), 2);
            }
            let result = match type_id {
                0 => operands.iter().sum(),
                1 => operands.iter().product(),
                2 => operands.iter().copied().min().unwrap(),
                3 => operands.iter().copied().max().unwrap(),
                5 => {
                    if operands[0] > operands[1] {
                        1
                    } else {
                        0
                    }
                }
                6 => {
                    if operands[0] < operands[1] {
                        1
                    } else {
                        0
                    }
                }
                7 => {
                    if operands[0] == operands[1] {
                        1
                    } else {
                        0
                    }
                }
                _ => panic!(),
            };

            (offset, result)
        }
    }

    let mut lines = BufReader::new(File::open("inputs/16.txt").unwrap()).lines();
    let line = lines.next().unwrap().unwrap();
    let mut binary = String::new();
    for c in line.chars() {
        binary.push_str(&format!("{:04b}", c.to_digit(16).unwrap()));
    }

    let (_parsed_char_count, result) = parse_packet(&binary, 0);

    println!("Day 16 problem 2: {}", result);
}
