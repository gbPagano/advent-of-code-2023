use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use num_integer::lcm;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    // part_one(contents);
    part_two(contents);
}

#[derive(Debug)]
struct Node<'a> {
    left: &'a str,
    right: &'a str,
}


fn part_one(contents: String) {
    let lines: Vec<&str> = contents.split('\n').into_iter().collect();

    let mut directions = lines.get(0).unwrap().chars().into_iter().cycle();
    let lines = lines.get(2..).unwrap();

    let mut nodes: HashMap<&str, Node> = HashMap::new();
    
    for line in lines.into_iter().rev() {
        match line.split_whitespace().into_iter().collect::<Vec<&str>>().as_slice() {
            [source, _, left, right] => {
                let node = Node {
                    left: left.trim_matches(|c| ['(', ','].contains(&c)), 
                    right: right.trim_matches(|c| [')', ','].contains(&c)), 
                };
                nodes.insert(source, node);
            },
            _ => unreachable!(),
        }
    }
    
    let mut steps = 0;
    let mut current = "AAA";
    while current != "ZZZ" {
        steps += 1;
        let dir = directions.next().unwrap();
        let node = nodes.get(&current).unwrap();
        match dir {
            'R' => {
                current = node.right;
            },
            'L' => {
                current = node.left;
            },
            _ => unreachable!(),
        }
    }
    println!("{}", steps);
}


fn part_two(contents: String) {
    let lines: Vec<&str> = contents.split('\n').into_iter().collect();

    let mut directions = lines.get(0).unwrap().chars().into_iter().cycle();
    let lines = lines.get(2..).unwrap();

    let mut nodes: HashMap<&str, Node> = HashMap::new();
    
    let mut currents: Vec<&str> = Vec::new();
    for line in lines.into_iter().rev() {
        match line.split_whitespace().into_iter().collect::<Vec<&str>>().as_slice() {
            [source, _, left, right] => {
                let node = Node {
                    left: left.trim_matches(|c| ['(', ','].contains(&c)), 
                    right: right.trim_matches(|c| [')', ','].contains(&c)), 
                };
                nodes.insert(source, node);

                if source.chars().last().unwrap() == 'A' {
                    currents.push(source);
                }
            },
            _ => unreachable!(),
        }
    }


    let mut results: Vec<u64> = Vec::new();
    for item in &currents {
        let mut steps = 0;
        let mut current = *item;
        while !current.ends_with('Z') {
            steps += 1;
            let dir = directions.next().unwrap();
            let node = nodes.get(&current).unwrap();
            match dir {
                'R' => {
                    current = node.right;
                },
                'L' => {
                    current = node.left;
                },
                _ => unreachable!(),
            }
        }
        println!("{} -> {} = {}", item, current, steps);
        results.push(steps);
    }

    let mmc = results.iter().cloned().fold(1, lcm);
    println!("mmc: {}", mmc);   
}