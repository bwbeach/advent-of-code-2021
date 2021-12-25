use std::collections::{HashMap, HashSet};

use crate::types::{AdventResult, Answer, Day, DayPart};

/// A graph is represented as a mapping from a node to the set of
/// nodes you can reach from it.
///
/// Each edge of a digraph is listed in both directions.
///
#[derive(Debug, PartialEq)]
struct Graph<'a> {
    // May from a node to the places you can get to from it.
    edges: HashMap<&'a str, HashSet<&'a str>>,

    // Constant empty set to return from neighbors() when there are no neighbors.
    no_neighbors: HashSet<&'a str>,
}

impl<'a, 'b> Graph<'a> {
    fn empty() -> Graph<'a> {
        Graph {
            edges: HashMap::new(),
            no_neighbors: HashSet::new(),
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

    fn make_set<'a>(items: &[&'a str]) -> HashSet<&'a str> {
        items.iter().map(|&n| n).collect()
    }

    assert_eq!(&make_set(&["b"]), graph.neighbors("a"));
    assert_eq!(&make_set(&["a", "c"]), graph.neighbors("b"));
    assert_eq!(&make_set(&["b"]), graph.neighbors("c"));
}

/// Is the room big?
fn is_big(room: &str) -> bool {
    room.chars().next().unwrap().is_uppercase()
}

/// Function that tests whether it's OK to visit a node.
type CanVisitFn = fn(node: &str, stack: &Vec<&str>, have_repeated: bool) -> bool;

/// Counts the number of paths from the current room to "end" that
/// do to visit any of the rooms in the stack, or any new rooms
/// that are not big.
fn paths_to_end<'a, 'b>(
    cave: &'a Graph,
    current: &'a str,
    stack: &'b mut Vec<&'a str>,
    have_repeated: bool,
    can_visit: CanVisitFn,
) -> Answer {
    if current == "end" {
        1
    } else {
        let mut count = 0;
        for next in cave.neighbors(current) {
            let is_repeat = !is_big(next) && stack.contains(next);
            if can_visit(next, stack, have_repeated) {
                stack.push(next);
                count += paths_to_end(cave, next, stack, have_repeated || is_repeat, can_visit);
                stack.pop();
            }
        }
        count
    }
}

fn day_12_a(lines: &Vec<String>) -> AdventResult<Answer> {
    let graph = parse_graph(lines);
    Ok(paths_to_end(
        &graph,
        "start",
        &mut vec!["start"],
        false,
        |node, stack, _| is_big(node) || !stack.contains(&node),
    ))
}

fn day_12_b(lines: &Vec<String>) -> AdventResult<Answer> {
    let graph = parse_graph(lines);
    Ok(paths_to_end(
        &graph,
        "start",
        &mut vec!["start"],
        false,
        |node, stack, have_repeated| {
            node != "start" && (is_big(node) || !have_repeated || !stack.contains(&node))
        },
    ))
}

pub fn make_day_12() -> Day {
    Day::new(
        12,
        DayPart::new(day_12_a, 10, 4792),
        DayPart::new(day_12_b, 36, 133360),
    )
}
