use std::collections::{HashMap, HashSet};
use std::convert::identity;
use std::fs::File;
use std::io::{BufRead, BufReader};
use traitgraph::implementation::petgraph_impl;
use traitgraph::index::GraphIndex;
use traitgraph::interface::{ImmutableGraphContainer, MutableGraphContainer, StaticGraph};

pub fn collect_all_paths<Graph: StaticGraph<NodeData = bool>>(
    graph: &Graph,
    start: Graph::NodeIndex,
    end: Graph::NodeIndex,
    allowed: &[u8],
    stack: &[Graph::NodeIndex],
) -> HashSet<Vec<Graph::NodeIndex>> {
    let mut result = HashSet::new();
    if start == end {
        result.insert(stack.to_vec());
    } else {
        let mut allowed = allowed.to_vec();
        if !graph.node_data(start) {
            allowed[start.as_usize()] -= 1;
        }
        let mut stack = stack.to_vec();
        stack.push(start);

        for neighbor_node in graph.out_neighbors(start).map(|n| n.node_id) {
            if allowed[neighbor_node.as_usize()] > 0 {
                result.extend(
                    collect_all_paths(graph, neighbor_node, end, &allowed, &stack).into_iter(),
                );
            }
        }
    }

    result
}

pub fn problem_1() {
    let mut graph = petgraph_impl::new();
    let mut node_map = HashMap::new();
    let mut start_node = None;
    let mut end_node = None;

    for line in BufReader::new(File::open("inputs/12.txt").unwrap()).lines() {
        let line = line.unwrap();
        let mut nodes = line.split('-');
        let n1_label = nodes.next().unwrap().to_owned();
        let n2_label = nodes.next().unwrap().to_owned();

        let n1 = if let Some(node_id) = node_map.get(&n1_label) {
            *node_id
        } else {
            let node_id = graph.add_node(n1_label.chars().map(|c| c.is_uppercase()).any(identity));
            node_map.insert(n1_label.clone(), node_id);
            node_id
        };

        let n2 = if let Some(node_id) = node_map.get(&n2_label) {
            *node_id
        } else {
            let node_id = graph.add_node(n2_label.chars().map(|c| c.is_uppercase()).any(identity));
            node_map.insert(n2_label.clone(), node_id);
            node_id
        };

        graph.add_edge(n1, n2, ());
        graph.add_edge(n2, n1, ());

        if n1_label == "start" {
            start_node = Some(n1);
        } else if n1_label == "end" {
            end_node = Some(n1);
        }
        if n2_label == "start" {
            start_node = Some(n2);
        } else if n2_label == "end" {
            end_node = Some(n2);
        }
    }

    let start_node = start_node.unwrap();
    let end_node = end_node.unwrap();

    let allowed = vec![1; graph.node_count()];
    let result = collect_all_paths(&graph, start_node, end_node, &allowed, &[]).len();

    println!("Day 12 problem 1: {}", result);
}

pub fn problem_2() {
    let mut graph = petgraph_impl::new();
    let mut node_map = HashMap::new();
    let mut start_node = None;
    let mut end_node = None;

    for line in BufReader::new(File::open("inputs/12.txt").unwrap()).lines() {
        let line = line.unwrap();
        let mut nodes = line.split('-');
        let n1_label = nodes.next().unwrap().to_owned();
        let n2_label = nodes.next().unwrap().to_owned();

        let n1 = if let Some(node_id) = node_map.get(&n1_label) {
            *node_id
        } else {
            let node_id = graph.add_node(n1_label.chars().map(|c| c.is_uppercase()).any(identity));
            node_map.insert(n1_label.clone(), node_id);
            node_id
        };

        let n2 = if let Some(node_id) = node_map.get(&n2_label) {
            *node_id
        } else {
            let node_id = graph.add_node(n2_label.chars().map(|c| c.is_uppercase()).any(identity));
            node_map.insert(n2_label.clone(), node_id);
            node_id
        };

        graph.add_edge(n1, n2, ());
        graph.add_edge(n2, n1, ());

        if n1_label == "start" {
            start_node = Some(n1);
        } else if n1_label == "end" {
            end_node = Some(n1);
        }
        if n2_label == "start" {
            start_node = Some(n2);
        } else if n2_label == "end" {
            end_node = Some(n2);
        }
    }

    let start_node = start_node.unwrap();
    let end_node = end_node.unwrap();

    let mut allowed = vec![1; graph.node_count()];
    let mut result = HashSet::new();

    for node_index in 0..graph.node_count() {
        if node_index == start_node.as_usize() || node_index == end_node.as_usize() {
            continue;
        }

        allowed[node_index] += 1;
        result.extend(collect_all_paths(&graph, start_node, end_node, &allowed, &[]).into_iter());
        allowed[node_index] -= 1;
    }

    println!("Day 12 problem 2: {}", result.len());
}
