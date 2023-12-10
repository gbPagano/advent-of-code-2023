use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    // part_one(contents);
    part_two(contents);
}

fn extrapolate(numbers: Vec<i32>) -> i32 {
    if numbers.iter().all(|x| x == &0) {
        0
    } else {
        let diffs: Vec<i32> = numbers.windows(2).map(|pair| pair[1] - pair[0]).collect();
        numbers.last().unwrap() + extrapolate(diffs)
    }
}

fn part_one(contents: String) {
    let lines: Vec<&str> = contents.split('\n').into_iter().collect();

    let mut result = 0;
    for line in lines {
        let numbers = line
            .split_whitespace()
            .into_iter()
            .map(|item| item.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let next_value = extrapolate(numbers);
        result += next_value;
    }

    println!("result: {result}");
}

fn part_two(contents: String) {
    let lines: Vec<&str> = contents.split('\n').into_iter().collect();

    let mut result = 0;
    for line in lines {
        let numbers = line
            .split_whitespace()
            .into_iter()
            .rev()
            .map(|item| item.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let next_value = extrapolate(numbers);
        result += next_value;
    }

    println!("result: {result}");
}
