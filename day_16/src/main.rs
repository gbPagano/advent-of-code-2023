use std::collections::HashSet;
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
    let grid = contents
        .split("\n")
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut seen: HashSet<(usize, usize, Direction)> = HashSet::new();
    let start_dir = match grid[0][0] {
        '\\' | '|' => Direction::Down,
        '/' => Direction::Up,
        '-' | '.' => Direction::Right,
        _ => unreachable!(),
    };

    move_beam(&grid, &mut seen, (0, 0, start_dir));

    let uniques = seen.iter().map(|(i, j, _)| (i, j)).collect::<HashSet<_>>();

    println!("{:?}", uniques.len());
}

fn part_two(contents: String) {
    let grid = contents
        .split("\n")
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut res = 0;
    for i in 0..grid.len() {
        for dir in [
            Direction::Left,
            Direction::Right,
        ] {
            res = res.max(energize_beam(&grid, (i, 0, dir)));
            res = res.max(energize_beam(&grid, (i, grid.len() - 1, dir)));
        }
    }
    for j in 0..grid[0].len() {
        for dir in [
            Direction::Up,
            Direction::Down,
        ] {
            res = res.max(energize_beam(&grid, (0, j, dir)));
            res = res.max(energize_beam(&grid, (grid[0].len() - 1, j, dir)));
        }
    }
    println!("res: {res}");
}

fn energize_beam(grid: &Vec<Vec<char>>, start: (usize, usize, Direction)) -> usize {
    let mut seen: HashSet<(usize, usize, Direction)> = HashSet::new();

    move_beam(grid, &mut seen, start);

    let uniques = seen.iter().map(|(i, j, _)| (i, j)).collect::<HashSet<_>>();

    uniques.len()
}

fn move_beam(
    grid: &Vec<Vec<char>>,
    seen: &mut HashSet<(usize, usize, Direction)>,
    current: (usize, usize, Direction),
) {
    let (i, j, dir) = current;
    match grid[i][j] {
        '\\' | '|' | '-' | '/' => {
            if seen.contains(&current) {
                return;
            }
        }
        _ => (),
    }
    seen.insert(current);

    match dir {
        Direction::Right => {
            if j + 1 < grid[0].len() {
                match grid[i][j + 1] {
                    '-' | '.' => move_beam(grid, seen, (i, j + 1, dir)),
                    '\\' => move_beam(grid, seen, (i, j + 1, Direction::Down)),
                    '|' => {
                        move_beam(grid, seen, (i, j + 1, Direction::Down));
                        move_beam(grid, seen, (i, j + 1, Direction::Up));
                    }
                    '/' => move_beam(grid, seen, (i, j + 1, Direction::Up)),
                    _ => unreachable!(),
                }
            }
        }
        Direction::Left => {
            if j > 0 {
                match grid[i][j - 1] {
                    '-' | '.' => move_beam(grid, seen, (i, j - 1, dir)),
                    '\\' => move_beam(grid, seen, (i, j - 1, Direction::Up)),
                    '|' => {
                        move_beam(grid, seen, (i, j - 1, Direction::Down));
                        move_beam(grid, seen, (i, j - 1, Direction::Up));
                    }
                    '/' => move_beam(grid, seen, (i, j - 1, Direction::Down)),
                    _ => unreachable!(),
                }
            }
        }
        Direction::Up => {
            if i > 0 {
                match grid[i - 1][j] {
                    '|' | '.' => move_beam(grid, seen, (i - 1, j, dir)),
                    '\\' => move_beam(grid, seen, (i - 1, j, Direction::Left)),
                    '-' => {
                        move_beam(grid, seen, (i - 1, j, Direction::Left));
                        move_beam(grid, seen, (i - 1, j, Direction::Right));
                    }
                    '/' => move_beam(grid, seen, (i - 1, j, Direction::Right)),
                    _ => unreachable!(),
                }
            }
        }
        Direction::Down => {
            if i + 1 < grid.len() {
                match grid[i + 1][j] {
                    '|' | '.' => move_beam(grid, seen, (i + 1, j, dir)),
                    '\\' => move_beam(grid, seen, (i + 1, j, Direction::Right)),
                    '-' => {
                        move_beam(grid, seen, (i + 1, j, Direction::Left));
                        move_beam(grid, seen, (i + 1, j, Direction::Right));
                    }
                    '/' => move_beam(grid, seen, (i + 1, j, Direction::Left)),
                    _ => unreachable!(),
                }
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
