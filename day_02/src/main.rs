// PART 1 
// use std::fs::File;
// use std::io::{self, BufRead};

// fn main() {
//     let file = File::open("input.txt").unwrap();
//     let reader = io::BufReader::new(file);

//     let mut result = 0;
//     'lines_for: for line in reader.lines() {
//         let items: Vec<String> = line.unwrap().split_whitespace().map(String::from).collect();
//         for (idx, value) in items.iter().enumerate() {
//             match value.as_str() {
//                 "red" | "red," | "red;" => {
//                     if items[idx - 1].parse::<i32>().unwrap() > 12 {
//                         continue 'lines_for;
//                     }
//                 },
//                 "blue" | "blue," | "blue;" => {
//                     if items[idx - 1].parse::<i32>().unwrap() > 14 {
//                         continue 'lines_for;
//                     }
//                 },
//                 "green" | "green," | "green;" => {
//                     if items[idx - 1].parse::<i32>().unwrap() > 13 {
//                         continue 'lines_for;
//                     }
//                 },
//                 _ => (),
//             }
//         }
//         result += items[1].trim_matches(|c| c == ':').parse::<i32>().unwrap();
//     }
//     println!("{result}");
// }

// PART 2
use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;


fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = io::BufReader::new(file);

    let mut result = 0;
    for line in reader.lines() {
        let items: Vec<String> = line.unwrap().split_whitespace().map(String::from).collect();
        let mut max_values: HashMap<&str, i32> = HashMap::new();

        
        for (idx, value) in items.iter().enumerate() {
            match value.as_str() {
                "red" | "red," | "red;" => match max_values.get("red") {
                    Some(max) => {
                        let value = items[idx - 1].parse::<i32>().unwrap();
                        if max < &value {
                            max_values.insert("red", value);
                        }
                    },
                    None => {
                        let value = items[idx - 1].parse::<i32>().unwrap();
                        max_values.insert("red", value);
                    },
                },
                "blue" | "blue," | "blue;" => match max_values.get("blue") {
                    Some(max) => {
                        let value = items[idx - 1].parse::<i32>().unwrap();
                        if max < &value {
                            max_values.insert("blue", value);
                        }
                    },
                    None => {
                        let value = items[idx - 1].parse::<i32>().unwrap();
                        max_values.insert("blue", value);
                    },
                },
                "green" | "green," | "green;" => match max_values.get("green") {
                    Some(max) => {
                        let value = items[idx - 1].parse::<i32>().unwrap();
                        if max < &value {
                            max_values.insert("green", value);
                        }
                    },
                    None => {
                        let value = items[idx - 1].parse::<i32>().unwrap();
                        max_values.insert("green", value);
                    },
                },
                _ => (),
            }
        }
        println!("{:?}", max_values.values());
        result += max_values.values().fold(1, |acc, valor| acc * valor);
        // result += items[1].trim_matches(|c| c == ':').parse::<i32>().unwrap();
    }
    println!("{result}");
}
