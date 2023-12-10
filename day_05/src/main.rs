// PART 1
// use std::fs::File;
// use std::io::Read;

// fn main() {
//     let mut file = File::open("input.txt").unwrap();
//     let mut contents = String::new();
//     file.read_to_string(&mut contents).unwrap();

//     let blocks: Vec<&str> = contents.split("\n\n").into_iter().collect();

//     let mut seeds = vec![0];
//     for block in blocks {
//         match block
//             .split(":")
//             .into_iter()
//             .collect::<Vec<&str>>()
//             .as_slice()
//         {
//             [category, ranges] => {
//                 if category == &"seeds" {
//                     seeds = ranges
//                         .split_whitespace()
//                         .into_iter()
//                         .map(|x| x.parse::<i64>().unwrap())
//                         .collect::<Vec<i64>>();
//                 } else {
//                     let numbers = ranges
//                         .split_whitespace()
//                         .into_iter()
//                         .map(|x| x.parse::<i64>().unwrap())
//                         .collect::<Vec<i64>>();

//                     let mut changed = vec![false; seeds.len()];
//                     for chunk in numbers.chunks(3) {
//                         match chunk {
//                             [dest_start, source_start, len_range] => {
//                                 for i in 0..seeds.len() {
//                                     let seed = &mut seeds[i];
//                                     if !changed[i]
//                                         && *seed >= *source_start
//                                         && *seed < source_start + len_range
//                                     {
//                                         print!("{} -> ", seed);
//                                         *seed = *seed + (dest_start - source_start);
//                                         println!("{}", seed);
//                                         changed[i] = true;
//                                         // println!("{:?}", );
//                                     }
//                                 }
//                             }
//                             _ => unreachable!(),
//                         }
//                     }

//                     println!("{:?}", seeds);
//                 }
//             }
//             _ => unreachable!(),
//         }
//     }
//     println!("{:?}", seeds.iter().min().unwrap());
// }

// PART 2
use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let blocks: Vec<&str> = contents.split("\n\n").into_iter().collect();

    let mut seeds: Vec<(i64, i64)> = Vec::new();
    for block in blocks {
        match block
            .split(":")
            .into_iter()
            .collect::<Vec<&str>>()
            .as_slice()
        {
            [category, ranges] => {
                if category == &"seeds" {
                    let seeds_ranges = ranges
                        .split_whitespace()
                        .into_iter()
                        .map(|x| x.parse::<i64>().unwrap())
                        .collect::<Vec<i64>>();
                    for chunk in seeds_ranges.chunks(2) {
                        match chunk {
                            [start, length] => {
                                seeds.push((*start, start + length - 1));
                            }
                            _ => unreachable!(),
                        }
                    }
                } else {
                    let numbers = ranges
                        .split_whitespace()
                        .into_iter()
                        .map(|x| x.parse::<i64>().unwrap())
                        .collect::<Vec<i64>>();

                    let mut i = 0;
                    while i < seeds.len() {
                        for range in numbers.chunks(3) {
                            let (start, end) = seeds[i];
                            match range {
                                [dest_start, source_start, len_range] => {
    
                                    let source_end = source_start + len_range - 1;
                                    
                                    let max_min = std::cmp::max(start, *source_start);
                                    let min_max = std::cmp::min(end, source_end);

                                    if max_min <= min_max {
                                        
                                        if start < max_min {
                                            seeds.push((start, max_min - 1))
                                        }
                                        if end > min_max {
                                            seeds.push((min_max + 1, end));
                                        }
                                                
                                        seeds[i].0 = max_min + dest_start - source_start;
                                        seeds[i].1 = min_max + dest_start - source_start;
                                        break;
                                    }
                                            
                                }
                                _ => unreachable!(),
                            }
                        }
                        i += 1;
                    }
                }
            }
            _ => unreachable!(),
        }
    }
    println!("{:?}", seeds.iter().min().unwrap());
}
