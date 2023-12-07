// PART 1
// use std::fs::File;
// use std::io::{self, BufRead};

// fn main() {
//     let file = File::open("input.txt").unwrap();
//     let reader = io::BufReader::new(file);

//     let mut result = 0;
//     for line in reader.lines() {
//         match line.unwrap().split('|').collect::<Vec<&str>>().as_slice() {
//             [winners, elf_cards] => {
                
//                 let winners_numbers = winners
//                     .split_whitespace()
//                     .collect::<Vec<&str>>()
//                     .iter()
//                     .filter_map(|x| {
//                         if x.chars().all(|c| c.is_digit(10)) {
//                             Some(x.parse::<i32>().unwrap())
//                         }
//                         else {
//                             None
//                         }
//                     })
//                     .collect::<Vec<i32>>();

//                 let elf_numbers = elf_cards
//                     .split_whitespace()
//                     .collect::<Vec<&str>>()
//                     .iter()
//                     .filter_map(|x| {
//                         if x.chars().all(|c| c.is_digit(10)) {
//                             Some(x.parse::<i32>().unwrap())
//                         }
//                         else {
//                             None
//                         }
//                     })
//                     .collect::<Vec<i32>>();


//                 let r = winners_numbers
//                     .iter()
//                     .filter(|num| elf_numbers.contains(&num))
//                     .count();

//                 println!("r: {:?}", r);
//                 match r {
//                     0 => (),
//                     1 => result += 1,
//                     _ => result += 2_u32.pow(r as u32 - 1)
//                 }
    
//             }
//             _ => unreachable!(),
//         }
//     }
//     println!("{}", result);
// }

// PART 2
use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let lines: Vec<&str> = contents.split('\n').into_iter().collect();
    
    let mut instances: Vec<u32>= vec![1; lines.len()];
    let mut result = 0;

    for (idx, line) in lines.iter().enumerate() {
        match line.split('|').collect::<Vec<&str>>().as_slice() {
            [winners, elf_cards] => {
                
                let winners_numbers = winners
                    .split_whitespace()
                    .collect::<Vec<&str>>()
                    .iter()
                    .filter_map(|x| {
                        if x.chars().all(|c| c.is_digit(10)) {
                            Some(x.parse::<i32>().unwrap())
                        }
                        else {
                            None
                        }
                    })
                    .collect::<Vec<i32>>();

                let elf_numbers = elf_cards
                    .split_whitespace()
                    .collect::<Vec<&str>>()
                    .iter()
                    .filter_map(|x| {
                        if x.chars().all(|c| c.is_digit(10)) {
                            Some(x.parse::<i32>().unwrap())
                        }
                        else {
                            None
                        }
                    })
                    .collect::<Vec<i32>>();


                let r = winners_numbers
                    .iter()
                    .filter(|num| elf_numbers.contains(&num))
                    .count();

                for i in idx+2..=idx+1+r {
                    instances[i-1] += instances[idx];
                    println!("{}, {}", i, i-1);
                }

                print!("idx: {:?} ", idx + 1);
                println!("r: {:?}", r);
                match r {
                    0 => (),
                    1 => result += 1,
                    _ => result += 2_u32.pow(r as u32 - 1)
                }
    
            }
            _ => unreachable!(),
        }
    }
    println!("{}", result);
    println!("{:?}", instances);
    println!("{:?}", instances.iter().sum::<u32>());
}