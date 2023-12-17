use cached::proc_macro::cached;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    // part_one(contents);
    part_two(contents);
}

fn part_one(contents: String) {
    let lines = contents
        .split("\n")
        .map(|item| item.split_whitespace().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut res = 0;
    for line in lines {
        let [spr, val] = line.as_slice() else {
            unreachable!()
        };
        let spring = spr.chars().collect::<Vec<char>>();
        let values = val
            .split(',')
            .map(|item| item.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        res += calc(spring, values);
    }
    println!("{:?}", res);
}

fn part_two(contents: String) {
    let lines = contents
        .split("\n")
        .map(|item| item.split_whitespace().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut res = 0;
    for line in lines {
        let [spr, val] = line.as_slice() else {
            unreachable!()
        };
        let spring = vec![*spr; 5].join("?").chars().collect::<Vec<char>>();
        let values = vec![*val; 5]
            .join(",")
            .split(',')
            .map(|item| item.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        res += calc(spring, values);
    }
    println!("{:?}", res);
}

#[cached]
fn calc(spring: Vec<char>, values: Vec<usize>) -> usize {
    if spring.is_empty() {
        if values.is_empty() {
            return 1;
        }
        return 0;
    }
    if values.is_empty() {
        if spring.contains(&'#') {
            return 0;
        }
        return 1;
    }

    let mut res = 0;

    // println!("{:?}  {:?}", spring, values);

    if ['.', '?'].contains(&spring[0]) {
        res += calc(spring[1..].to_vec(), values.clone());
    }

    if ['#', '?'].contains(&spring[0])
        && values[0] <= spring.len()
        && !spring[..values[0]].contains(&'.')
        && (values[0] == spring.len() || spring[values[0]] != '#')
    {
        if values[0] < spring.len() {
            res += calc(spring[values[0] + 1..].to_vec(), values[1..].to_vec());
        } else {
            res += calc(spring[values[0]..].to_vec(), values[1..].to_vec());
        }
    }

    res
}
