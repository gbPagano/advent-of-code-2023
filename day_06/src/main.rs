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
    let lines: Vec<&str> = contents.split("\n").into_iter().collect();
    let [line_time, line_distance] = lines.as_slice() else { unreachable!() };
    
    let times = line_time
        .split_whitespace()
        .filter_map(|x| {
            if x.chars().all(|c| c.is_digit(10)) {
                Some(x.parse::<i32>().unwrap())
            }
            else {
                None
            }
        })
        .collect::<Vec<i32>>();

    let distances = line_distance
        .split_whitespace()
        .filter_map(|x| {
            if x.chars().all(|c| c.is_digit(10)) {
                Some(x.parse::<i32>().unwrap())
            }
            else {
                None
            }
        })
        .collect::<Vec<i32>>();

    let mut result = 0;
    for (time, distance) in times.iter().zip(distances) {
        println!("{:?}, {:?}", time, distance);
        let mut wins = 0;
        for hold_seconds in 1..*time {
            let travel = hold_seconds * (time - hold_seconds);
            if travel > distance {
                wins += 1;
            }
        }
        println!("wins {}", wins);
        match result {
            0 => result = wins,
            _ => result *= wins,
        }
    }
    println!("res {}", result);
}


fn part_two(contents: String) {
    let lines: Vec<&str> = contents.split("\n").into_iter().collect();
    let [line_time, line_distance] = lines.as_slice() else { unreachable!() };
    
    let time = line_time
        .split_whitespace()
        .filter(|x| x.chars().all(|c| c.is_digit(10)) )
        .collect::<Vec<&str>>()
        .join("")
        .parse::<i64>()
        .unwrap();

    let distance = line_distance
        .split_whitespace()
        .filter(|x| x.chars().all(|c| c.is_digit(10)) )
        .collect::<Vec<&str>>()
        .join("")
        .parse::<i64>()
        .unwrap();

    println!("{time:?}");
    println!("{distance:?}");

    let mut wins = 0;
    for hold_seconds in 1..time {
        let travel = hold_seconds * (time - hold_seconds);
        if travel > distance {
            wins += 1;
        }
    }
    println!("wins {}", wins);
}