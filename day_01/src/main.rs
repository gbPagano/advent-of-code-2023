// PART 1
// use std::fs::File;
// use std::io::{self, BufRead};

// fn main() {
//     let file = File::open("input.txt").unwrap();
//     let reader = io::BufReader::new(file);

//     let mut result = 0;
//     for line in reader.lines() {
//         let mut number = String::new();
//         for letter in line.unwrap().chars() {
//             if letter.is_digit(10) {
//                 number.push(letter);
//             }
//         }

//         let mut first_and_last = String::new();
//         first_and_last.push(number.chars().nth(0).unwrap());
//         first_and_last.push(number.chars().last().unwrap());

//         result += first_and_last.parse::<i32>().unwrap();

//     }
//     println!("{:?}", result);
// }

// PART 2
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = io::BufReader::new(file);

    let mut result = 0;
    for line in reader.lines() {
        let mut line = line.unwrap();
        println!("{:?}", line);
        line = line.replace("one", "o1e");
        line = line.replace("two", "t2o");
        line = line.replace("three", "t3e");
        line = line.replace("four", "f4r");
        line = line.replace("five", "f5e");
        line = line.replace("six", "s6x");
        line = line.replace("seven", "s7n");
        line = line.replace("eight", "e8t");
        line = line.replace("nine", "n9e");
        // if line.contains("eight") {
            // line = line.replace("eight", "8");
        // }

        let mut number = String::new();
        for letter in line.chars() {
            if letter.is_digit(10) {
                number.push(letter);
            }
        }

        let mut first_and_last = String::new();
        first_and_last.push(number.chars().nth(0).unwrap());
        first_and_last.push(number.chars().last().unwrap());

        println!("{:?}", line);
        println!("{:?}", first_and_last);
        result += first_and_last.parse::<i32>().unwrap();

    }
    println!("{:?}", result);
}