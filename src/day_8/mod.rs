use std::fs::File;
use std::io::{BufRead, BufReader};
use varisat::{ExtendFormula, Lit, Solver};

pub fn problem_1() {
    let result: usize = BufReader::new(File::open("inputs/8.txt").unwrap())
        .lines()
        .map(|line| {
            line.unwrap()
                .split('|')
                .nth(1)
                .unwrap()
                .trim()
                .split(' ')
                .filter(|n| n.len() == 2 || n.len() == 3 || n.len() == 4 || n.len() == 7)
                .count()
        })
        .sum();

    println!("Day 8 problem 1: {}", result);
}

pub fn problem_2() {
    fn add_clause(solver: &mut Solver, clause: &[Lit]) {
        //println!("add_clause: {:?}", clause);
        solver.add_clause(clause)
    }

    fn encode_char(c: char) -> isize {
        match c {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            _ => panic!(),
        }
    }

    fn decode_char(i: isize) -> char {
        match i {
            0 => 'a',
            1 => 'b',
            2 => 'c',
            3 => 'd',
            4 => 'e',
            5 => 'f',
            6 => 'g',
            _ => panic!(),
        }
    }

    fn encode_var(from: char, to: char) -> Lit {
        Lit::from_dimacs(encode_char(from) * 7 + encode_char(to) + 1)
    }

    let mut result = 0;
    for (line_number, line) in BufReader::new(File::open("inputs/8.txt").unwrap())
        .lines()
        .enumerate()
    {
        let line_number = line_number + 1;
        let line = line.unwrap();
        let line = line.trim();
        let columns: Vec<_> = line.split('|').map(str::trim).collect();
        let digits: Vec<_> = columns[0].split(' ').collect();
        let all_chars = ['a', 'b', 'c', 'd', 'e', 'f', 'g'];

        let mut solver = Solver::new();

        for from in all_chars {
            // Each char needs to be mapped
            add_clause(
                &mut solver,
                &all_chars
                    .iter()
                    .map(|&to| encode_var(from, to))
                    .collect::<Vec<_>>(),
            );
            // Each char needs to be mapped to
            add_clause(
                &mut solver,
                &all_chars
                    .iter()
                    .map(|&to| encode_var(to, from))
                    .collect::<Vec<_>>(),
            );

            for (offset, &to1) in all_chars.iter().enumerate() {
                for &to2 in all_chars.iter().skip(offset + 1) {
                    // Each char must not be mapped more than once
                    add_clause(
                        &mut solver,
                        &[!encode_var(from, to1), !encode_var(from, to2)],
                    );
                    // Each char must not be mapped to more than once
                    add_clause(
                        &mut solver,
                        &[!encode_var(to1, from), !encode_var(to2, from)],
                    );
                }
            }
        }

        for digit in digits {
            match digit.len() {
                2 => {
                    // Number 1
                    add_clause(
                        &mut solver,
                        &digit
                            .chars()
                            .map(|c| encode_var(c, 'c'))
                            .collect::<Vec<_>>(),
                    );
                    add_clause(
                        &mut solver,
                        &digit
                            .chars()
                            .map(|c| encode_var(c, 'f'))
                            .collect::<Vec<_>>(),
                    );
                }
                3 => {
                    // Number 7
                    add_clause(
                        &mut solver,
                        &digit
                            .chars()
                            .map(|c| encode_var(c, 'a'))
                            .collect::<Vec<_>>(),
                    );
                    add_clause(
                        &mut solver,
                        &digit
                            .chars()
                            .map(|c| encode_var(c, 'c'))
                            .collect::<Vec<_>>(),
                    );
                    add_clause(
                        &mut solver,
                        &digit
                            .chars()
                            .map(|c| encode_var(c, 'f'))
                            .collect::<Vec<_>>(),
                    );
                }
                4 => {
                    // Number 4
                    add_clause(
                        &mut solver,
                        &digit
                            .chars()
                            .map(|c| encode_var(c, 'b'))
                            .collect::<Vec<_>>(),
                    );
                    add_clause(
                        &mut solver,
                        &digit
                            .chars()
                            .map(|c| encode_var(c, 'c'))
                            .collect::<Vec<_>>(),
                    );
                    add_clause(
                        &mut solver,
                        &digit
                            .chars()
                            .map(|c| encode_var(c, 'd'))
                            .collect::<Vec<_>>(),
                    );
                    add_clause(
                        &mut solver,
                        &digit
                            .chars()
                            .map(|c| encode_var(c, 'f'))
                            .collect::<Vec<_>>(),
                    );
                }
                5 => {
                    // Numbers 2, 3 and 5
                    // Shared mappings
                    add_clause(
                        &mut solver,
                        &digit
                            .chars()
                            .map(|c| encode_var(c, 'a'))
                            .collect::<Vec<_>>(),
                    );
                    add_clause(
                        &mut solver,
                        &digit
                            .chars()
                            .map(|c| encode_var(c, 'd'))
                            .collect::<Vec<_>>(),
                    );
                    add_clause(
                        &mut solver,
                        &digit
                            .chars()
                            .map(|c| encode_var(c, 'g'))
                            .collect::<Vec<_>>(),
                    );

                    // Inferred conditions
                    for (offset, from1) in digit.chars().enumerate() {
                        for from2 in digit.chars().skip(offset + 1) {
                            // b and c are never shared
                            add_clause(
                                &mut solver,
                                &[!encode_var(from1, 'b'), !encode_var(from2, 'c')],
                            );
                            // b and e are never shared
                            add_clause(
                                &mut solver,
                                &[!encode_var(from1, 'b'), !encode_var(from2, 'e')],
                            );
                            // e and f are never shared
                            add_clause(
                                &mut solver,
                                &[!encode_var(from1, 'e'), !encode_var(from2, 'f')],
                            );
                        }

                        // b implies f
                        add_clause(
                            &mut solver,
                            &([!encode_var(from1, 'b')]
                                .into_iter()
                                .chain(digit.chars().map(|from2| encode_var(from2, 'f')))
                                .collect::<Vec<_>>()),
                        );
                        // c implies e or f
                        add_clause(
                            &mut solver,
                            &([!encode_var(from1, 'c')]
                                .into_iter()
                                .chain(
                                    digit
                                        .chars()
                                        .map(|from2| {
                                            [encode_var(from2, 'e'), encode_var(from2, 'f')]
                                                .into_iter()
                                        })
                                        .flatten(),
                                )
                                .collect::<Vec<_>>()),
                        );
                        // e implies c
                        add_clause(
                            &mut solver,
                            &([!encode_var(from1, 'e')]
                                .into_iter()
                                .chain(digit.chars().map(|from2| encode_var(from2, 'c')))
                                .collect::<Vec<_>>()),
                        );
                        // f implies b or c
                        add_clause(
                            &mut solver,
                            &([!encode_var(from1, 'f')]
                                .into_iter()
                                .chain(
                                    digit
                                        .chars()
                                        .map(|from2| {
                                            [encode_var(from2, 'b'), encode_var(from2, 'c')]
                                                .into_iter()
                                        })
                                        .flatten(),
                                )
                                .collect::<Vec<_>>()),
                        );
                    }
                }
                6 => {
                    // Numbers 0, 6 and 9
                    // Shared mappings
                    add_clause(
                        &mut solver,
                        &digit
                            .chars()
                            .map(|c| encode_var(c, 'a'))
                            .collect::<Vec<_>>(),
                    );
                    add_clause(
                        &mut solver,
                        &digit
                            .chars()
                            .map(|c| encode_var(c, 'b'))
                            .collect::<Vec<_>>(),
                    );
                    add_clause(
                        &mut solver,
                        &digit
                            .chars()
                            .map(|c| encode_var(c, 'f'))
                            .collect::<Vec<_>>(),
                    );
                    add_clause(
                        &mut solver,
                        &digit
                            .chars()
                            .map(|c| encode_var(c, 'g'))
                            .collect::<Vec<_>>(),
                    );

                    // Inferred conditions: none
                }
                7 => {
                    // Number 8
                }
                _ => panic!(),
            }
        }

        let satisfiable = solver.solve().unwrap();
        assert!(
            satisfiable,
            "NOT SATISFIABLE! line {}: {}",
            line_number, line
        );
        let model = solver.model().unwrap();

        let mapping: Vec<_> = (0..7)
            .map(|from| {
                (0..7)
                    .filter(|&to| model.contains(&encode_var(decode_char(from), decode_char(to))))
                    .next()
                    .unwrap()
            })
            .collect();
        let mut number = 0;
        let mut exponent = 1;

        for scrambled_digit in columns[1].split(' ').rev() {
            let mut real_digit: Vec<_> = scrambled_digit
                .chars()
                .map(|c| decode_char(mapping[encode_char(c) as usize]))
                .collect();
            real_digit.sort();

            number += exponent
                * match real_digit.as_slice() {
                    ['a', 'b', 'c', 'e', 'f', 'g'] => 0,
                    ['c', 'f'] => 1,
                    ['a', 'c', 'd', 'e', 'g'] => 2,
                    ['a', 'c', 'd', 'f', 'g'] => 3,
                    ['b', 'c', 'd', 'f'] => 4,
                    ['a', 'b', 'd', 'f', 'g'] => 5,
                    ['a', 'b', 'd', 'e', 'f', 'g'] => 6,
                    ['a', 'c', 'f'] => 7,
                    ['a', 'b', 'c', 'd', 'e', 'f', 'g'] => 8,
                    ['a', 'b', 'c', 'd', 'f', 'g'] => 9,
                    _ => panic!(
                        "MAPPING ERROR! line {}: {}\nmapping: {:?}",
                        line_number, line, mapping
                    ),
                };

            exponent *= 10;
        }

        result += number;
    }

    println!("Day 8 problem 2: {}", result);
}
