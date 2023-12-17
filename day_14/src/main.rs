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
    let mut lever = contents
        .split_whitespace()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for line in &lever {
        println!("{:?}", line);
    }
    println!("");

    tilt_up(&mut lever);

    let mut res = 0;
    for (idx, line) in lever.iter().enumerate() {
        let value = lever.len() - idx;
        let rocks = line.iter().filter(|c| *c == &'O').count();
        println!("{:?} {} {}", line, value, rocks);
        res += rocks * value;
    }
    println!("res: {res}");
}

fn part_two(contents: String) {
    let mut lever = contents
        .split_whitespace()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for line in &lever {
        println!("{:?}", line);
    }
    println!("");

    let mut seen: Vec<Vec<Vec<char>>> = Vec::new();
    let mut last = 0;
    loop {
        if seen.contains(&lever) {
            break;
        }
        last += 1;
        seen.push(lever.clone());
        cycle(&mut lever);
    }
    let first = seen.iter().position(|x| *x == lever).unwrap();
    let final_lever = &seen[(1_000_000_000 - first) % (last - first) + first];

    let mut res = 0;
    for (idx, line) in final_lever.iter().enumerate() {
        let value = final_lever.len() - idx;
        let rocks = line.iter().filter(|c| *c == &'O').count();
        res += rocks * value;
    }
    println!("res: {res}");
}

fn cycle(lever: &mut Vec<Vec<char>>) {
    // north
    tilt_up(lever);
    // weast
    tilt_left(lever);
    // south
    tilt_down(lever);
    // east
    tilt_right(lever);
    // back to original pos
}

fn tilt_left(lever: &mut Vec<Vec<char>>) {
    for line in lever {
        for idx in 1..line.len() {
            let mut current = idx;
            if line[idx] == 'O' {
                while current > 0 && line[current - 1] == '.' {
                    line[current] = '.';
                    line[current - 1] = 'O';
                    current -= 1;
                }
            }
        }
    }
}

fn tilt_right(lever: &mut Vec<Vec<char>>) {
    for line in lever {
        for idx in (0..line.len()).rev() {
            let mut current = idx;
            if line[idx] == 'O' {
                while current + 1 < line.len() && line[current + 1] == '.' {
                    line[current] = '.';
                    line[current + 1] = 'O';
                    current += 1;
                }
            }
        }
    }
}

fn tilt_up(lever: &mut Vec<Vec<char>>) {
    let n_cols = lever[0].len();
    for col_idx in 0..n_cols {
        for line_idx in 1..lever.len() {
            let mut current = line_idx;
            if lever[line_idx][col_idx] == 'O' {
                while current > 0 && lever[current - 1][col_idx] == '.' {
                    lever[current][col_idx] = '.';
                    lever[current - 1][col_idx] = 'O';
                    current -= 1;
                }
            }
        }
    }
}

fn tilt_down(lever: &mut Vec<Vec<char>>) {
    let n_cols = lever[0].len();
    for col_idx in 0..n_cols {
        for line_idx in (0..lever.len()).rev() {
            let mut current = line_idx;
            if lever[line_idx][col_idx] == 'O' {
                while current + 1 < lever.len() && lever[current + 1][col_idx] == '.' {
                    lever[current][col_idx] = '.';
                    lever[current + 1][col_idx] = 'O';
                    current += 1;
                }
            }
        }
    }
}
