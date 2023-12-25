use rand::Rng;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

fn main() {
    let contents = fs::read_to_string("test.txt").unwrap();

    part_one(contents);
}

fn part_one(contents: String) {
    let mut graph: HashMap<&str, HashSet<&str>> = HashMap::new();
    for line in contents.split("\n") {
        let (k, values) = line.split_once(": ").unwrap();
        for v in values.split_whitespace() {
            graph.entry(k).or_default().insert(v);
            graph.entry(v).or_default().insert(k);
        }
    }

    let mut removed: Vec<(&str, &str)> = Vec::new();
    for _ in 0..3 {
        let bridge = find_bridge(&graph);
        graph.get_mut(&bridge.0).unwrap().remove(bridge.1);
        graph.get_mut(&bridge.1).unwrap().remove(bridge.0);
        removed.push(bridge);
    }
    for bridge in removed {
        let size_a = count(&graph, bridge.0);
        let size_b = count(&graph, bridge.1);
        if size_a == size_b {
            continue;
        }
        let part_two = size_a * size_b;
        dbg!(part_two);
        break;
    }
}

fn count<'a>(graph: &HashMap<&'a str, HashSet<&'a str>>, start: &'a str) -> usize {
    let mut res = 1;
    let mut to_see = VecDeque::new();
    let mut seen = HashSet::new();
    to_see.push_back(start);
    seen.insert(start);
    while let Some(node) = to_see.pop_front() {
        for n in graph[node].iter() {
            if !seen.contains(n) {
                to_see.push_back(n);
                seen.insert(n);

                res += 1;
            }
        }
    }
    res
}

fn find_bridge<'a>(graph: &HashMap<&'a str, HashSet<&'a str>>) -> (&'a str, &'a str) {
    let mut paths: HashMap<(&str, &str), usize> = HashMap::new();
    let mut rng = rand::thread_rng();

    for start in graph.keys() {
        if rng.gen_range(1..=10) == 1 {
            continue;
        }
        let mut to_see = VecDeque::new();
        let mut seen = HashSet::new();
        to_see.push_back(*start);
        seen.insert(*start);
        while let Some(node) = to_see.pop_front() {
            for n in graph[node].iter() {
                if !seen.contains(n) {
                    to_see.push_back(n);
                    seen.insert(n);

                    let edge = (node.min(n), node.max(n)); // (a, b) == (b, a)
                    *paths.entry(edge).or_default() += 1;
                }
            }
        }
    }
    paths.into_iter().max_by_key(|&(_, v)| v).unwrap().0
}
