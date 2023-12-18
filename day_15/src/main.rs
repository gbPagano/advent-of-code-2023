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
    let res = contents
        .trim()
        .split(",")
        .map(|item| {
            item.chars()
                .fold(0, |acc, item| ((acc + item as usize) * 17) % 256)
        })
        .sum::<usize>();

    println!("{:?}", res);
}

fn part_two(contents: String) {
    let items = contents.trim().split(",").collect::<Vec<_>>();

    let mut boxes: Vec<Vec<(&str, usize)>> = vec![Vec::new(); 256];
    for item in items {
        if item.chars().last().unwrap() == '-' {
            let label = item.strip_suffix("-").unwrap();
            let hashed_label = label
                .chars()
                .fold(0, |acc, item| ((acc + item as usize) * 17) % 256);

            if let Some(index) = boxes[hashed_label].iter().position(|(l, _)| *l == label) {
                boxes[hashed_label].remove(index);
            }
        } else {
            let [label, focal_length] = item.split("=").collect::<Vec<_>>()[..] else {
                unreachable!()
            };
            let hashed_label = label
                .chars()
                .fold(0, |acc, item| ((acc + item as usize) * 17) % 256);

            if let Some(index) = boxes[hashed_label].iter().position(|(l, _)| *l == label) {
                boxes[hashed_label][index] = (label, focal_length.parse::<usize>().unwrap());
            } else {
                boxes[hashed_label].push((label, focal_length.parse::<usize>().unwrap()));
            }
        }
    }
    let res = boxes
        .iter()
        .enumerate()
        .map(|(i, vec)| {
            vec.iter()
                .enumerate()
                .map(|(j, (_, focal_length))| (i + 1) * (j + 1) * focal_length)
                .sum::<usize>()
        })
        .sum::<usize>();

    println!("boxes: {:?}", res);
}
