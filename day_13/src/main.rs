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
    let mirrors = contents
        .split("\n\n")
        .map(|paragraph| paragraph.split('\n').map(|s| s.chars().collect::<Vec<_>>()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut res = 0;
    for (i, mirror) in mirrors.iter().enumerate() {
        println!("{i}");
        if let Some(above) = is_horizontal(mirror) {
            println!("horizontal with {} above", above);
            res += 100 * above;
        }
        if let Some(left) = is_vertical(mirror) {
            println!("vertical with {} left", left);
            res += left;
        }
    }
    println!("res: {}", res);
}

fn part_two(contents: String) {
    let mirrors = contents
        .split("\n\n")
        .map(|paragraph| paragraph.split('\n').map(|s| s.chars().collect::<Vec<_>>()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut res = 0;
    for (i, mirror) in mirrors.iter().enumerate() {
        println!("{i}");
        if let Some(above) = is_horizontal_2(mirror) {
            println!("horizontal with {} above", above);
            res += 100 * above;
        } 
        if let Some(left) = is_vertical_2(mirror) {
            println!("vertical with {} left", left);
            res += left;
        }
    }
    println!("res: {}", res);
}

fn is_horizontal(mirror: &Vec<Vec<char>>) -> Option<usize>{
    'window_loop: for (idx, window) in mirror.windows(2).enumerate() {
        let line_a = &window[0];
        let line_b = &window[1];

        if line_a == line_b {
            for (i, j) in (0..idx).rev().zip(idx+2..mirror.len()) {
                if mirror[i] != mirror[j] {
                    continue 'window_loop;
                }
            }
            let above = idx + 1;
            return Some(above);
        }
    }
    None
}

fn is_vertical(mirror: &Vec<Vec<char>>) -> Option<usize> {
    let num_cols = mirror[0].len();

    'column_loop: for (a, b) in (0..num_cols - 1).zip(1..num_cols) {
        if mirror.iter().all(|line| line[a] == line[b]) {
            for (i, j) in (0..a).rev().zip(a+2..num_cols) {
                if !mirror.iter().all(|line| line[i] == line[j]) {
                    continue 'column_loop;
                }
            }
            let left = b;
            return Some(left);
        }
    }
    None
}

fn is_horizontal_2(mirror: &Vec<Vec<char>>) -> Option<usize>{
    'window_loop: for (idx, window) in mirror.windows(2).enumerate() {
        let line_a = &window[0];
        let line_b = &window[1];

        let mut smudge = false;
        let equals = line_a.iter().zip(line_b).filter(|(a,b)| a == b).count();
        if equals == line_a.len() - 1 {
            smudge = true;
        }

        if equals == line_a.len() || smudge {

            for (i, j) in (0..idx).rev().zip(idx+2..mirror.len()) {
                let equals = mirror[i].iter().zip(&mirror[j]).filter(|(a, b)| a == b).count();
                if equals != line_a.len() {
                    let diff = line_a.len().abs_diff(equals);
                    if smudge || diff > 1{
                        continue 'window_loop;
                    }
                    smudge = true;
                }
            }
            if smudge {
                let above = idx + 1;
                return Some(above);
            }
        }
    }
    None
}

fn is_vertical_2(mirror: &Vec<Vec<char>>) -> Option<usize> {
    let num_cols = mirror[0].len();

    'column_loop: for (a, b) in (0..num_cols - 1).zip(1..num_cols) {
        let mut smudge = false;
        let equals = mirror.iter().filter(|line| line[a] == line[b]).count();
        if equals == mirror.len() - 1 {
            smudge = true;
        }
        
        if equals == mirror.len() || smudge {
            for (i, j) in (0..a).rev().zip(a+2..num_cols) {
                let equals = mirror.iter().filter(|line| line[i] == line[j]).count();
                if equals != mirror.len() {
                    let diff = mirror.len().abs_diff(equals);
                    if smudge || diff > 1 {
                        continue 'column_loop;
                    }
                    smudge = true;
                }
            }
            if smudge {
                let left = b;
                return Some(left);
            }
        }
    }
    None
}
