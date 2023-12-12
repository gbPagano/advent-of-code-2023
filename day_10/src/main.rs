use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    part_one(contents);
    // part_two(contents);
}

fn part_one(contents: String) {
    let grid = contents
        .split('\n')
        .into_iter()
        .map(|item| item.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut start_pos: (usize, usize) = (0, 0);
    let mut distance_grid = vec![vec![i64::MAX; grid[0].len()]; grid.len()];
    for (i, line) in grid.iter().enumerate() {
        // println!("{:?}   {:?}", line, distance_grid[i]);
        for (j, tile) in line.iter().enumerate() {
            if tile == &'S' {
                start_pos = (i, j);
                distance_grid[i][j] = 0;
            } else if tile == &'.' {
                distance_grid[i][j] = 0;
            }
        }
    }

    evaluate_steps(&grid, &mut distance_grid, start_pos, 0);
    // for (i, line) in grid.iter().enumerate() {
    //     println!("{:?}   {:?}", line, distance_grid[i]);
    // }
    let res = distance_grid.iter().flat_map(|row| row.iter()).filter(|&&x| x < i64::MAX).max().unwrap();
    println!("res: {}", res);
}

fn get_upper(
    grid: &Vec<Vec<char>>,
    distance_grid: &Vec<Vec<i64>>,
    current_pos: (usize, usize),
    next_positions: &mut Vec<(usize, usize)>,
    current_step: i64,
) {
    let valid_chars = ['|', 'F', '7'];
    match current_pos {
        (0, _) => (), // index out of limits
        (i, j) => {
            if let Some(_) = grid
                .get(i - 1)
                .and_then(|r| r.get(j))
                .filter(|&&c| valid_chars.contains(&c))
            {
                if distance_grid[i - 1][j] > current_step {
                    next_positions.push((i - 1, j));
                }
            }
        }
    }
}

fn get_left(
    grid: &Vec<Vec<char>>,
    distance_grid: &Vec<Vec<i64>>,
    current_pos: (usize, usize),
    next_positions: &mut Vec<(usize, usize)>,
    current_step: i64,
) {
    let valid_chars = ['-', 'F', 'L'];
    match current_pos {
        (_, 0) => (), // index out of limits
        (i, j) => {
            if let Some(_) = grid
                .get(i)
                .and_then(|r| r.get(j - 1))
                .filter(|&&c| valid_chars.contains(&c))
            {
                if distance_grid[i][j - 1] > current_step {
                    next_positions.push((i, j - 1));
                }
            }
        }
    }
    
}

fn get_lower(
    grid: &Vec<Vec<char>>,
    distance_grid: &Vec<Vec<i64>>,
    current_pos: (usize, usize),
    next_positions: &mut Vec<(usize, usize)>,
    current_step: i64,
) {
    let valid_chars = ['|', 'J', 'L'];
    let (i, j) = current_pos;
    if let Some(_) = grid
        .get(i + 1)
        .and_then(|r| r.get(j))
        .filter(|&&c| valid_chars.contains(&c))
    {
        if distance_grid[i + 1][j] > current_step {
            next_positions.push((i + 1, j));
        }
    }
}

fn get_right(
    grid: &Vec<Vec<char>>,
    distance_grid: &Vec<Vec<i64>>,
    current_pos: (usize, usize),
    next_positions: &mut Vec<(usize, usize)>,
    current_step: i64,
) {
    let valid_chars = ['-', '7', 'J'];
    let (i, j) = current_pos;
    if let Some(_) = grid
        .get(i)
        .and_then(|r| r.get(j + 1))
        .filter(|&&c| valid_chars.contains(&c))
    {
        if distance_grid[i][j + 1] > current_step {
            next_positions.push((i, j + 1));
        }
    }
}

fn evaluate_steps(
    grid: &Vec<Vec<char>>,
    distance_grid: &mut Vec<Vec<i64>>,
    current_pos: (usize, usize),
    steps: i64,
) {
    let (i, j) = current_pos;
    if distance_grid[i][j] > steps {
        distance_grid[i][j] = steps;
    }

    let mut next_positions: Vec<(usize, usize)> = Vec::new();

    // pega a pos de cima
    let current = grid[i][j];
    // println!("{current}");
    match current {
        'S' => {
            get_upper(grid, &distance_grid, current_pos, &mut next_positions, steps);
            get_lower(grid, distance_grid, current_pos, &mut next_positions, steps);
            get_left(grid, distance_grid, current_pos, &mut next_positions, steps);
            get_right(grid, distance_grid, current_pos, &mut next_positions, steps);
        }
        '|' => {
            get_upper(grid, distance_grid, current_pos, &mut next_positions, steps);
            get_lower(grid, distance_grid, current_pos, &mut next_positions, steps);
        },
        '-' => {
            get_left(grid, distance_grid, current_pos, &mut next_positions, steps);
            get_right(grid, distance_grid, current_pos, &mut next_positions, steps);
        },
        'L' => {
            get_upper(grid, distance_grid, current_pos, &mut next_positions, steps);
            get_right(grid, distance_grid, current_pos, &mut next_positions, steps);
        },
        'F' => {
            get_lower(grid, distance_grid, current_pos, &mut next_positions, steps);
            get_right(grid, distance_grid, current_pos, &mut next_positions, steps);
        },
        'J' => {
            get_upper(grid, distance_grid, current_pos, &mut next_positions, steps);
            get_left(grid, distance_grid, current_pos, &mut next_positions, steps);
        },
        '7' => {
            get_lower(grid, distance_grid, current_pos, &mut next_positions, steps);
            get_left(grid, distance_grid, current_pos, &mut next_positions, steps);
        },
        _ => (),
    }

    // println!("{:?}", current_pos);
    println!("{:?}", next_positions);
    // for (i, line) in grid.iter().enumerate() {
    //     println!("{:?}   {:?}", line, distance_grid[i]);
    // }
    // pega a pos de baixo
    // pega a pos da direita
    // pega a pos da esquerda

    for pos in next_positions {
        evaluate_steps(grid, distance_grid, pos, steps + 1);
    }
}
