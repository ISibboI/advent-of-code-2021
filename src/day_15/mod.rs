use std::fs::File;
use std::io::{BufRead, BufReader};
use traitgraph::algo::dijkstra::DefaultDijkstra;
use traitgraph::implementation::petgraph_impl;
use traitgraph::interface::{ImmutableGraphContainer, MutableGraphContainer};

pub fn problem_1() {
    let mut graph = petgraph_impl::new();
    let mut previous_nodes: Option<Vec<(_, _)>> = None;
    for line in BufReader::new(File::open("inputs/15.txt").unwrap()).lines() {
        let line = line.unwrap();
        let mut nodes = Vec::new();
        let mut previous_node = None;
        for (offset, digit) in line.chars().map(|c| c.to_digit(10).unwrap()).enumerate() {
            let node = graph.add_node(());
            if let Some((previous_node, previous_digit)) = previous_node {
                graph.add_edge(previous_node, node, digit);
                graph.add_edge(node, previous_node, previous_digit);
            }
            if let Some(previous_nodes) = previous_nodes.as_ref() {
                graph.add_edge(previous_nodes[offset].0, node, digit);
                graph.add_edge(node, previous_nodes[offset].0, previous_nodes[offset].1);
            }
            previous_node = Some((node, digit));
            nodes.push((node, digit));
        }
        previous_nodes = Some(nodes);
    }

    let mut dijkstra = DefaultDijkstra::new(&graph);
    let first_node = graph.node_indices().nth(0).unwrap();
    let last_node = graph.node_indices().last().unwrap();
    let mut distances = Vec::new();
    dijkstra.shortest_path_lens(
        &graph,
        first_node,
        &last_node,
        1,
        u32::MAX,
        false,
        &mut distances,
    );

    println!("Day 15 problem 1: {}", distances[0].1);
}

pub fn problem_2() {
    let mut graph = petgraph_impl::new();
    let mut previous_nodes: Option<Vec<(_, _)>> = None;
    let lines: Vec<_> = BufReader::new(File::open("inputs/15.txt").unwrap())
        .lines()
        .map(Result::unwrap)
        .collect();
    for (repetition, line) in (0..5)
        .map(|repetition| lines.iter().cloned().map(move |line| (repetition, line)))
        .flatten()
    {
        let mut nodes = Vec::new();
        let mut previous_node = None;
        for (offset, digit) in (0..5)
            .map(|line_repetition| {
                line.chars().map(move |c| {
                    let result = c.to_digit(10).unwrap() + repetition + line_repetition;
                    if result > 9 {
                        result - 9
                    } else {
                        result
                    }
                })
            })
            .flatten()
            .enumerate()
        {
            assert!(digit < 10);
            let node = graph.add_node(());
            if let Some((previous_node, previous_digit)) = previous_node {
                graph.add_edge(previous_node, node, digit);
                graph.add_edge(node, previous_node, previous_digit);
            }
            if let Some(previous_nodes) = previous_nodes.as_ref() {
                graph.add_edge(previous_nodes[offset].0, node, digit);
                graph.add_edge(node, previous_nodes[offset].0, previous_nodes[offset].1);
            }
            previous_node = Some((node, digit));
            nodes.push((node, digit));
        }
        previous_nodes = Some(nodes);
    }

    let mut dijkstra = DefaultDijkstra::new(&graph);
    let first_node = graph.node_indices().nth(0).unwrap();
    let last_node = graph.node_indices().last().unwrap();
    let mut distances = Vec::new();
    dijkstra.shortest_path_lens(
        &graph,
        first_node,
        &last_node,
        1,
        u32::MAX,
        false,
        &mut distances,
    );

    println!("Day 15 problem 2: {}", distances[0].1);
}
