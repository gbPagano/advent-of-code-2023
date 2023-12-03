// PART 1
// use std::fs::File;
// use std::io::Read;
// use regex::Regex;


// fn main() {
//     let mut file = File::open("input.txt").unwrap();

//     let mut contents = String::new();
//     file.read_to_string(&mut contents).unwrap();
//     println!("{}\n\n", contents);

//     let lines: Vec<&str> = contents.split_whitespace().into_iter().collect();

//     let mut result = 0;
//     let re = Regex::new(r"\b\d+\b").unwrap();
//     for (i, line) in lines.iter().enumerate() {
//         println!("{}", line);
//         'number_for: for number in re.find_iter(&line) {
//             // println!("{:?}", number.as_str());
//             // println!("{:?}", number.as_str().parse::<i32>().unwrap());
//             for j in number.start()..number.end() {
//                 if i > 0 {
//                     if lines.get(i - 1).unwrap().chars().nth(j) != Some('.') {
//                         println!("{:?}", number.as_str());
//                         result += number.as_str().parse::<i32>().unwrap();
//                         continue 'number_for;
//                     }
//                 }
//                 if i < lines.len() - 1 {
//                     if lines.get(i + 1).unwrap().chars().nth(j) != Some('.') {
//                         println!("{:?}", number.as_str());
//                         result += number.as_str().parse::<i32>().unwrap();
//                         continue 'number_for;
//                     }
//                 }
//                 if j > 0 {
//                     if i > 0 {
//                         if lines.get(i - 1).unwrap().chars().nth(j - 1) != Some('.') {
//                             println!("{:?}", number.as_str());
//                             result += number.as_str().parse::<i32>().unwrap();
//                             continue 'number_for;
//                         }
//                     }
//                     if i < lines.len() - 1 {
//                         if lines.get(i + 1).unwrap().chars().nth(j - 1) != Some('.') {
//                             println!("{:?}", number.as_str());
//                             result += number.as_str().parse::<i32>().unwrap();
//                             continue 'number_for;
//                         }
//                     }
//                     if let Some(item) = lines.get(i).unwrap().chars().nth(j - 1) {
//                         if item != '.' && !item.is_digit(10) {
//                             println!("{:?}", number.as_str());
//                             result += number.as_str().parse::<i32>().unwrap();
//                             continue 'number_for;
//                         }
//                     }
//                 }
//                 if j < line.len() - 1 {
//                     if i > 0 {
//                         if lines.get(i - 1).unwrap().chars().nth(j + 1) != Some('.') {
//                             println!("{:?}", number.as_str());
//                             result += number.as_str().parse::<i32>().unwrap();
//                             continue 'number_for;
//                         }
//                     }
//                     if i < lines.len() - 1 {
//                         if lines.get(i + 1).unwrap().chars().nth(j + 1) != Some('.') {
//                             println!("{:?}", number.as_str());
//                             result += number.as_str().parse::<i32>().unwrap();
//                             continue 'number_for;
//                         }
//                     }
//                     if let Some(item) = lines.get(i).unwrap().chars().nth(j + 1) {
//                         if item != '.' && !item.is_digit(10) {
//                             println!("{:?}", number.as_str());
//                             result += number.as_str().parse::<i32>().unwrap();
//                             continue 'number_for;
//                         }
//                     }
//                 }
//             }
//         }
//     }

//     println!("{}", result);
// }

// PART 2
use std::fs::File;
use std::io::Read;
use regex::Regex;
use std::collections::HashMap;

fn main() {
    let mut file = File::open("input.txt").unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}\n\n", contents);

    let lines: Vec<&str> = contents.split_whitespace().into_iter().collect();

    let mut my_map: HashMap<(usize, usize), Vec<i32>> = HashMap::new();
    let re = Regex::new(r"\b\d+\b").unwrap();
    for (i, line) in lines.iter().enumerate() {
        println!("{}", line);
        'number_for: for number in re.find_iter(&line) {
            for j in number.start()..number.end() {
                if i > 0 {
                    if lines.get(i - 1).unwrap().chars().nth(j) == Some('*') {
                        match my_map.entry((i - 1, j)) {
                            std::collections::hash_map::Entry::Occupied(mut entry) => {
                                entry.get_mut().push(number.as_str().parse::<i32>().unwrap());
                            }
                            std::collections::hash_map::Entry::Vacant(entry) => {
                                entry.insert(vec![number.as_str().parse::<i32>().unwrap()]);
                            }
                        }    

                        println!("{:?}", number.as_str());
                        continue 'number_for;
                    }
                }
                if i < lines.len() - 1 {
                    if lines.get(i + 1).unwrap().chars().nth(j) == Some('*') {
                        match my_map.entry((i + 1, j)) {
                            std::collections::hash_map::Entry::Occupied(mut entry) => {
                                entry.get_mut().push(number.as_str().parse::<i32>().unwrap());
                            }
                            std::collections::hash_map::Entry::Vacant(entry) => {
                                entry.insert(vec![number.as_str().parse::<i32>().unwrap()]);
                            }
                        }    

                        println!("{:?}", number.as_str());
                        continue 'number_for;
                    }
                }
                if j > 0 {
                    if i > 0 {
                        if lines.get(i - 1).unwrap().chars().nth(j - 1) == Some('*') {
                            match my_map.entry((i - 1, j - 1)) {
                                std::collections::hash_map::Entry::Occupied(mut entry) => {
                                    entry.get_mut().push(number.as_str().parse::<i32>().unwrap());
                                }
                                std::collections::hash_map::Entry::Vacant(entry) => {
                                    entry.insert(vec![number.as_str().parse::<i32>().unwrap()]);
                                }
                            }    
    
                            println!("{:?}", number.as_str());
                            continue 'number_for;
                        }
                    }
                    if i < lines.len() - 1 {
                        if lines.get(i + 1).unwrap().chars().nth(j - 1) == Some('*') {
                            match my_map.entry((i + 1, j - 1)) {
                                std::collections::hash_map::Entry::Occupied(mut entry) => {
                                    entry.get_mut().push(number.as_str().parse::<i32>().unwrap());
                                }
                                std::collections::hash_map::Entry::Vacant(entry) => {
                                    entry.insert(vec![number.as_str().parse::<i32>().unwrap()]);
                                }
                            }    
    
                            println!("{:?}", number.as_str());
                            continue 'number_for;
                        }
                    }
                    if lines.get(i).unwrap().chars().nth(j - 1) == Some('*') {
                        match my_map.entry((i, j - 1)) {
                            std::collections::hash_map::Entry::Occupied(mut entry) => {
                                entry.get_mut().push(number.as_str().parse::<i32>().unwrap());
                            }
                            std::collections::hash_map::Entry::Vacant(entry) => {
                                entry.insert(vec![number.as_str().parse::<i32>().unwrap()]);
                            }
                        }    

                        println!("{:?}", number.as_str());
                        continue 'number_for;
                    }
                }
                if j < line.len() - 1 {
                    if i > 0 {
                        if lines.get(i - 1).unwrap().chars().nth(j + 1) == Some('*') {
                            match my_map.entry((i - 1, j + 1)) {
                                std::collections::hash_map::Entry::Occupied(mut entry) => {
                                    entry.get_mut().push(number.as_str().parse::<i32>().unwrap());
                                }
                                std::collections::hash_map::Entry::Vacant(entry) => {
                                    entry.insert(vec![number.as_str().parse::<i32>().unwrap()]);
                                }
                            }    
    
                            println!("{:?}", number.as_str());
                            continue 'number_for;
                        }
                    }
                    if i < lines.len() - 1 {
                        if lines.get(i + 1).unwrap().chars().nth(j + 1) == Some('*') {
                            match my_map.entry((i + 1, j + 1)) {
                                std::collections::hash_map::Entry::Occupied(mut entry) => {
                                    entry.get_mut().push(number.as_str().parse::<i32>().unwrap());
                                }
                                std::collections::hash_map::Entry::Vacant(entry) => {
                                    entry.insert(vec![number.as_str().parse::<i32>().unwrap()]);
                                }
                            }    
    
                            println!("{:?}", number.as_str());
                            continue 'number_for;
                        }
                    }
                    if lines.get(i).unwrap().chars().nth(j + 1) == Some('*') {
                        match my_map.entry((i, j + 1)) {
                            std::collections::hash_map::Entry::Occupied(mut entry) => {
                                entry.get_mut().push(number.as_str().parse::<i32>().unwrap());
                            }
                            std::collections::hash_map::Entry::Vacant(entry) => {
                                entry.insert(vec![number.as_str().parse::<i32>().unwrap()]);
                            }
                        }    

                        println!("{:?}", number.as_str());
                        continue 'number_for;
                    }
                }
            }
        }
    }

    let mut result = 0;
    for values in my_map.values() {
        if values.len() == 2 {
            println!("{:?}", values);
            result += values[0] * values[1];
        }
    }
    println!("{:?}", result);
}
