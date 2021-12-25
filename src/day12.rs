use std::collections::{HashMap, HashSet};

use crate::types::{AdventResult, Answer, Day, DayPart};

/// A graph is represented as a mapping from a node to the set of
/// nodes you can reach from it.
///
/// Each edge of a digraph is listed in both directions.
///
#[derive(Debug, PartialEq)]
struct Graph<'a> {
    edges: HashMap<&'a str, HashSet<&'a str>>,
    no_neighbors: HashSet<&'a str>,
}

impl<'a, 'b> Graph<'a> {
    fn empty() -> Graph<'a> {
        Graph {
            edges: HashMap::new(),
            no_neighbors: HashSet::new(),
        }
    }

    fn has_edge(&self, a: &str, b: &str) -> bool {
        match self.edges.get(a) {
            Some(targets) => targets.contains(b),
            None => false,
        }
    }

    fn neighbors(&'a self, a: &'b str) -> &'a HashSet<&'a str> {
        self.edges.get(a).unwrap_or(&self.no_neighbors)
    }

    fn add_undirected_edge(&'b mut self, x: &'a str, y: &'a str) {
        self.add_directed_edge(x, y);
        self.add_directed_edge(y, x);
    }

    fn add_directed_edge(&'b mut self, x: &'a str, y: &'a str) {
        self.edges
            .entry(x)
            .or_insert_with(|| HashSet::new())
            .insert(y);
    }
}

fn parse_graph<'a>(lines: &'a Vec<String>) -> Graph<'a> {
    let mut graph = Graph::empty();
    for line in lines {
        let parts: Vec<_> = line.split("-").collect();
        if parts.len() != 2 {
            panic!("Bad graph line: {:?}", line);
        }
        graph.add_undirected_edge(parts[0], parts[1]);
    }
    graph
}

#[test]
fn test_parse_graph() {
    let lines = vec!["a-b".to_string(), "c-b".to_string()];
    let graph = parse_graph(&lines);

    assert_eq!(true, graph.has_edge("a", "b"));
    assert_eq!(false, graph.has_edge("a", "c"));
    assert_eq!(true, graph.has_edge("b", "a"));
    assert_eq!(true, graph.has_edge("b", "c"));
    assert_eq!(false, graph.has_edge("c", "a"));
    assert_eq!(true, graph.has_edge("c", "b"));
}

/// Is the room big?
fn is_big(room: &str) -> bool {
    room.chars().next().unwrap().is_uppercase()
}

/// Counts the number of paths from the current room to "end" that
/// do to visit any of the rooms in the stack, or any new rooms
/// that are not big.
fn paths_to_end<'a, 'b>(cave: &'a Graph, current: &'a str, stack: &'b mut Vec<&'a str>) -> Answer {
    if current == "end" {
        1
    } else {
        let mut count = 0;
        for next in cave.neighbors(current) {
            if is_big(next) || !stack.contains(next) {
                stack.push(next);
                count += paths_to_end(cave, next, stack);
                stack.pop();
            }
        }
        count
    }
}

fn day_12_a(lines: &Vec<String>) -> AdventResult<Answer> {
    let graph = parse_graph(lines);
    Ok(paths_to_end(&graph, "start", &mut vec!["start"]))
}

fn day_12_b(_lines: &Vec<String>) -> AdventResult<Answer> {
    Ok(0)
}

pub fn make_day_12() -> Day {
    Day::new(
        12,
        DayPart::new(day_12_a, 10, 4792),
        DayPart::new(day_12_b, 0, 0),
    )
}
